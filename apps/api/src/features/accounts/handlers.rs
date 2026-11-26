use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use diceri_domain::{AccessKey, Username};
use serde::{Deserialize, Serialize};

use super::session::{CurrentAccount, session_cookie};
use crate::{ApiError, AppState};

#[derive(Debug, Deserialize)]
pub(crate) struct RegisterRequest {
    username: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SignInRequest {
    key: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AccountResponse {
    username: String,
}

/// Sign-up also returns the key, once, so the owner can sign in elsewhere.
#[derive(Debug, Serialize)]
pub(crate) struct RegisteredResponse {
    username: String,
    key: String,
}

fn account(username: &Username) -> AccountResponse {
    AccountResponse {
        username: username.to_string(),
    }
}

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let username: Username = body
        .username
        .parse()
        .map_err(|error| ApiError::Validation(format!("{error}")))?;
    let key = state.accounts.register(&username).await?;
    tracing::info!("account created");
    let body = RegisteredResponse {
        username: username.to_string(),
        key: key.expose().to_owned(),
    };
    Ok((
        StatusCode::CREATED,
        session_cookie(Some(&key), state.secure_cookies),
        Json(body),
    ))
}

pub(crate) async fn sign_in(
    State(state): State<AppState>,
    Json(body): Json<SignInRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let key: AccessKey = body
        .key
        .trim()
        .parse()
        .map_err(|_| ApiError::Unauthorized)?;
    let username = state.accounts.authenticate(&key).await?;
    Ok((
        session_cookie(Some(&key), state.secure_cookies),
        Json(account(&username)),
    ))
}

pub(crate) async fn me(CurrentAccount(username): CurrentAccount) -> Json<AccountResponse> {
    Json(account(&username))
}

pub(crate) async fn sign_out(State(state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::NO_CONTENT,
        session_cookie(None, state.secure_cookies),
    )
}
