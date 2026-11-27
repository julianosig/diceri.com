//! What this deployment offers, so the web app shows only what works.
//!
//! | Method | Path | Auth |
//! | --- | --- | --- |
//! | GET | `/capabilities` | — (`{ voiceCloning, plans }`) |

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use diceri_domain::Plan;
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CapabilitiesResponse {
    /// Recording your own voice for translations (a paid feature, off until it ships).
    voice_cloning: bool,
    /// Every plan and its limits, free first (everyone's default).
    plans: Vec<PlanResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlanResponse {
    plan: &'static str,
    call_minutes: u32,
    daily_minutes: u32,
    monthly_minutes: u32,
    /// US cents a month; 0 for free.
    monthly_price_cents: u32,
}

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/capabilities", get(capabilities))
}

async fn capabilities(State(state): State<AppState>) -> Json<CapabilitiesResponse> {
    let catalogue = state.metering.plans();
    let plans = Plan::ALL
        .into_iter()
        .map(|plan| {
            let limits = catalogue.limits(plan);
            PlanResponse {
                plan: plan.as_str(),
                call_minutes: limits.call_minutes,
                daily_minutes: limits.daily_minutes,
                monthly_minutes: limits.monthly_minutes,
                monthly_price_cents: limits.monthly_price_cents,
            }
        })
        .collect();
    Json(CapabilitiesResponse {
        voice_cloning: state.voices.is_some(),
        plans,
    })
}
