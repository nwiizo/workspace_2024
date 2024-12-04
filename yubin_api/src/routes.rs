use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use metrics::{counter, histogram};
use tracing::info;

use crate::{
    error::ApiError,
    models::{AddressQuery, AddressResponse},
};

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

/// Lookup address by postal code
pub async fn lookup_by_postal_code(
    Path(code): Path<String>,
) -> Result<Json<Vec<AddressResponse>>, ApiError> {
    info!("Looking up postal code: {}", code);
    counter!("yubin_api_postal_lookups_total", 1);
    let start = std::time::Instant::now();

    let result = jpostcode_rs::lookup_address(&code).map_err(|e| match e {
        jpostcode_rs::JPostError::InvalidFormat => ApiError::InvalidPostalCode,
        jpostcode_rs::JPostError::NotFound => ApiError::NotFound,
    })?;

    let duration = start.elapsed().as_secs_f64();
    histogram!("yubin_api_postal_lookup_duration_seconds", duration);

    Ok(Json(result.into_iter().map(Into::into).collect()))
}

/// Search addresses by query
pub async fn search_by_address(
    Json(query): Json<AddressQuery>,
) -> Result<Json<Vec<AddressResponse>>, ApiError> {
    info!("Searching address with query: {}", query.query);
    if query.query.trim().is_empty() {
        return Err(ApiError::InvalidPostalCode);
    }

    counter!("yubin_api_address_searches_total", 1);
    let start = std::time::Instant::now();

    let mut results: Vec<AddressResponse> = jpostcode_rs::search_by_address(&query.query)
        .into_iter()
        .map(Into::into)
        .collect();

    results.truncate(query.limit);

    let duration = start.elapsed().as_secs_f64();
    histogram!("yubin_api_address_search_duration_seconds", duration);

    Ok(Json(results))
}
