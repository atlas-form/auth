use std::sync::Arc;

use axum::{Extension, Json};
use service::api::auth::AuthApi;
use toolcraft_axum_kit::{ApiError, CommonOk, Empty, IntoCommonResponse, ResponseResult};
use toolcraft_jwt::Jwt;
use validator::Validate;

use crate::{
    dto::auth::{
        LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
    },
    error::Error,
    statics::db_manager::get_default_ctx,
};

fn svc(e: db_core::Error) -> ApiError {
    ApiError::from(Error::from(e))
}

fn jwt_err() -> ApiError {
    ApiError::from(Error::Custom("Token generation failed".to_owned()))
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Registered successfully", body = CommonOk),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Username or email already exists"),
    )
)]
pub async fn register(Json(req): Json<RegisterRequest>) -> ResponseResult<Empty> {
    req.validate()
        .map_err(Error::Validation)
        .map_err(ApiError::from)?;

    let api = AuthApi::new(get_default_ctx());
    api.register(service::dto::auth::RegisterRequest {
        username: req.username,
        display_name: req.display_name,
        avatar: req.avatar,
        password: req.password,
        email: req.email,
    })
    .await
    .map_err(svc)?;

    Ok(Empty.into_common_response().to_json())
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Invalid credentials or account disabled"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn login(
    Extension(jwt): Extension<Arc<Jwt>>,
    Json(req): Json<LoginRequest>,
) -> ResponseResult<LoginResponse> {
    req.validate()
        .map_err(Error::Validation)
        .map_err(ApiError::from)?;

    let api = AuthApi::new(get_default_ctx());
    let user = api
        .login(service::dto::auth::LoginRequest {
            identifier: req.identifier,
            password: req.password,
        })
        .await
        .map_err(svc)?;

    let token_pair = jwt
        .generate_token_pair_for_subject(user.id)
        .map_err(|_| jwt_err())?;

    Ok(LoginResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    post,
    path = "/refresh_token",
    tag = "Auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = RefreshTokenResponse),
        (status = 401, description = "Invalid or expired refresh token"),
    )
)]
pub async fn refresh_token(
    Extension(jwt): Extension<Arc<Jwt>>,
    Json(req): Json<RefreshTokenRequest>,
) -> ResponseResult<RefreshTokenResponse> {
    let access_token = jwt
        .refresh_access_token(&req.refresh_token)
        .map_err(|_| ApiError::from(Error::Custom("Invalid refresh token".to_owned())))?;

    Ok(RefreshTokenResponse { access_token }
        .into_common_response()
        .to_json())
}
