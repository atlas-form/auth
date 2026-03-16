use axum::{Router, middleware, routing::{get, post, put}};
use toolcraft_axum_kit::middleware::auth_mw;
use utoipa::OpenApi;

use crate::handlers::user::{me, update_email, update_password, verify_email};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user::me,
        crate::handlers::user::update_password,
        crate::handlers::user::update_email,
        crate::handlers::user::verify_email,
    ),
    tags((name = "User", description = "User management APIs")),
)]
pub struct UserApiDoc;

pub fn user_routes() -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/password", put(update_password))
        .route("/email", put(update_email))
        .route("/email/verify", post(verify_email))
        .route_layer(middleware::from_fn(auth_mw::auth))
}
