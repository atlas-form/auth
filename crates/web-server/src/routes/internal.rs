use axum::{Router, routing::get};

use crate::handlers::internal::jwt_verify_config;

pub fn internal_routes() -> Router {
    Router::new().route("/jwt_verify_config", get(jwt_verify_config))
}
