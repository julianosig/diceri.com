use axum::Json;
use std::net::SocketAddr;

use axum::Extension;
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use diceri_application::use_cases::{Registration, SignedIn};
use diceri_domain::{Account, Mascot, Password, Username};
use serde::{Deserialize, Serialize};

use super::session::{CurrentAccount, key_from_cookies, session_cookie};
use crate::{ApiError, AppState};

/// The single onboarding step. The web app asks for the password twice; the API
/// takes it once.
#[derive(Debug, Deserialize)]
pub(crate) struct RegisterRequest {
    username: String,
    password: String,
    mascot: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SignInRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AccountResponse {
    username: String,
    mascot: &'static str,
    plan: &'static str,
}

impl From<&Account> for AccountResponse {
    fn from(account: &Account) -> Self {
        Self {
            username: account.username.to_string(),
            mascot: account.mascot.as_str(),
            plan: account.plan.as_str(),
        }
    }
}

/// The signed-in account with its plan's limits and today's use, shown on the home page.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MeResponse {
    #[serde(flatten)]
    account: AccountResponse,
    /// A voice recording is kept for the account (ADR 0015), so calls can reuse it.
    has_voice: bool,
    usage: UsageResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UsageResponse {
    /// Call minutes used today (UTC), charged while someone is in the owner's call.
    used_today: u32,
    daily_minutes: u32,
    /// Call minutes used this calendar month (UTC), and what the plan includes.
    used_this_month: u32,
    monthly_minutes: u32,
    /// Longest single call.
    call_minutes: u32,
}

fn invalid(error: impl std::fmt::Display) -> ApiError {
    ApiError::Validation(error.to_string())
}

fn signed_in(state: &AppState, signed_in: &SignedIn) -> impl IntoResponse + use<> {
    (
        session_cookie(Some(&signed_in.token), state.secure_cookies),
        Json(AccountResponse::from(&signed_in.account)),
    )
}

/// Signs up; one account per network when a network key is configured.
pub(crate) async fn register(
    State(state): State<AppState>,
    connect: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // The peer is absent in in-process tests; `main` serves with connect info.
    let peer = connect.map(|Extension(ConnectInfo(addr))| addr.ip());
    let registration = Registration {
        network: state
            .signup_networks
            .as_ref()
            .and_then(|networks| networks.of(&headers, peer)),
        username: body.username.parse::<Username>().map_err(invalid)?,
        password: body.password.parse::<Password>().map_err(invalid)?,
        mascot: body.mascot.parse::<Mascot>().map_err(invalid)?,
    };
    let account = state.accounts.register(registration).await?;
    tracing::info!(account_id = %account.account.id, "account created");
    Ok((StatusCode::CREATED, signed_in(&state, &account)))
}

/// Wrong username and wrong password answer the same 401.
pub(crate) async fn sign_in(
    State(state): State<AppState>,
    Json(body): Json<SignInRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let username: Username = body.username.parse().map_err(|_| ApiError::Unauthorized)?;
    let password: Password = body.password.parse().map_err(|_| ApiError::Unauthorized)?;
    let account = state.accounts.sign_in(&username, &password).await?;
    Ok(signed_in(&state, &account))
}

#[derive(Debug, Deserialize)]
pub(crate) struct AvailabilityQuery {
    username: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AvailabilityResponse {
    /// The username as it would be stored (lowercase, without `@`).
    username: String,
    available: bool,
}

/// `GET /accounts/available?username=…`: lets onboarding continue only with a free name.
pub(crate) async fn availability(
    State(state): State<AppState>,
    Query(query): Query<AvailabilityQuery>,
) -> Result<Json<AvailabilityResponse>, ApiError> {
    let username: Username = query.username.parse().map_err(invalid)?;
    let available = state.accounts.is_available(&username).await?;
    Ok(Json(AvailabilityResponse {
        username: username.to_string(),
        available,
    }))
}

/// `GET /users/{username}`: the public profile behind `/@username`, so a room that is
/// not live can still show its mascot. Unclaimed or malformed names are a 404.
pub(crate) async fn profile(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<AccountResponse>, ApiError> {
    let username: Username = username.parse().map_err(|_| ApiError::NotFound)?;
    let account = state
        .accounts
        .profile(&username)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(AccountResponse::from(&account)))
}

/// The signed-in account and its use, or 204 No Content when nobody is signed in.
pub(crate) async fn me(
    State(state): State<AppState>,
    signed_in: Option<CurrentAccount>,
) -> Result<axum::response::Response, ApiError> {
    let Some(CurrentAccount(account)) = signed_in else {
        return Ok(StatusCode::NO_CONTENT.into_response());
    };
    let allowance = state.metering.allowance(&account).await?;
    let has_voice = match &state.voices {
        Some(voices) => voices.has_saved(&account.id).await?,
        None => false,
    };
    Ok(Json(MeResponse {
        account: AccountResponse::from(&account),
        has_voice,
        usage: UsageResponse {
            used_today: allowance.used_today,
            daily_minutes: allowance.limits.daily_minutes,
            used_this_month: allowance.used_this_month,
            monthly_minutes: allowance.limits.monthly_minutes,
            call_minutes: allowance.limits.call_minutes,
        },
    })
    .into_response())
}

/// "Delete my voice": removes the kept recording (ADR 0015); the next call asks for a new one.
pub(crate) async fn delete_voice(
    State(state): State<AppState>,
    CurrentAccount(account): CurrentAccount,
) -> Result<StatusCode, ApiError> {
    if let Some(voices) = &state.voices {
        voices.delete_saved(&account.id).await?;
    }
    tracing::info!(account_id = %account.id, "kept voice deleted");
    Ok(StatusCode::NO_CONTENT)
}

/// Ends this browser's session server-side and expires the cookie.
pub(crate) async fn sign_out(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(token) = key_from_cookies(&headers) {
        state.accounts.sign_out(&token).await?;
    }
    Ok((
        StatusCode::NO_CONTENT,
        session_cookie(None, state.secure_cookies),
    ))
}
