use axum::Json;
use axum::extract::State;
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
}

impl From<&Account> for AccountResponse {
    fn from(account: &Account) -> Self {
        Self {
            username: account.username.to_string(),
            mascot: account.mascot.as_str(),
        }
    }
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

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let registration = Registration {
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

pub(crate) async fn me(CurrentAccount(account): CurrentAccount) -> Json<AccountResponse> {
    Json(AccountResponse::from(&account))
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
