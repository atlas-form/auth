use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::handlers::auth::{login, refresh_token, register};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::register,
        crate::handlers::auth::login,
        crate::handlers::auth::refresh_token,
    ),
    tags((name = "Session", description = "Authentication session APIs")),
)]
pub struct SessionApiDoc;

pub fn session_routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh_token", post(refresh_token))
}
