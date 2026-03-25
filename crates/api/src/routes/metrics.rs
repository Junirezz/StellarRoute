//! Metrics endpoint

use axum::{extract::State, Json};
use std::sync::Arc;

use crate::{models::CacheMetricsResponse, state::AppState};

/// Cache metrics endpoint
#[utoipa::path(
    get,
    path = "/metrics/cache",
    tag = "health",
    responses(
        (status = 200, description = "Cache hit/miss metrics", body = CacheMetricsResponse),
    )
)]
pub async fn cache_metrics(State(state): State<Arc<AppState>>) -> Json<CacheMetricsResponse> {
    let (quote_hits, quote_misses) = state.cache_metrics.snapshot();

    Json(CacheMetricsResponse {
        quote_hits,
        quote_misses,
    })
}
