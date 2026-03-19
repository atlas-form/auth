use axum::{Extension, Json};
use service::api::auth::AuthApi;
use toolcraft_axum_kit::{
    ApiError, CommonOk, Empty, IntoCommonResponse, ResponseResult, middleware::auth_mw::AuthUser,
};
use validator::Validate;

use crate::{
    dto::auth::{UpdateEmailRequest, UpdatePasswordRequest, UserResponse},
    error::Error,
    statics::db_manager::get_default_ctx,
};

fn svc(e: db_core::Error) -> ApiError {
    ApiError::from(Error::from(e))
}

#[utoipa::path(
    get,
    path = "/me",
    tag = "User",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "Current user info", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn me(Extension(auth_user): Extension<AuthUser>) -> ResponseResult<UserResponse> {
    let api = AuthApi::new(get_default_ctx());
    let user = api.get_user(&auth_user.user_id).await.map_err(svc)?;

    Ok(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        email_verified: user.email_verified,
        disabled: user.disabled,
    }
    .into_common_response()
    .to_json())
}

#[utoipa::path(
    put,
    path = "/password",
    tag = "User",
    security(("Bearer" = [])),
    request_body = UpdatePasswordRequest,
    responses(
        (status = 200, description = "Password updated", body = CommonOk),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Wrong old password"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn update_password(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<UpdatePasswordRequest>,
) -> ResponseResult<Empty> {
    req.validate()
        .map_err(Error::Validation)
        .map_err(ApiError::from)?;

    let api = AuthApi::new(get_default_ctx());
    api.update_password(
        &auth_user.user_id,
        service::dto::auth::UpdatePasswordRequest {
            old_password: req.old_password,
            new_password: req.new_password,
        },
    )
    .await
    .map_err(svc)?;

    Ok(Empty.into_common_response().to_json())
}

#[utoipa::path(
    put,
    path = "/email",
    tag = "User",
    security(("Bearer" = [])),
    request_body = UpdateEmailRequest,
    responses(
        (status = 200, description = "Email updated", body = CommonOk),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already in use"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn update_email(
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<UpdateEmailRequest>,
) -> ResponseResult<Empty> {
    req.validate()
        .map_err(Error::Validation)
        .map_err(ApiError::from)?;

    let api = AuthApi::new(get_default_ctx());
    api.update_email(&auth_user.user_id, req.email)
        .await
        .map_err(svc)?;

    Ok(Empty.into_common_response().to_json())
}

#[utoipa::path(
    post,
    path = "/email/verify",
    tag = "User",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "Email verified", body = CommonOk),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn verify_email(Extension(auth_user): Extension<AuthUser>) -> ResponseResult<Empty> {
    let api = AuthApi::new(get_default_ctx());
    api.verify_email(&auth_user.user_id).await.map_err(svc)?;

    Ok(Empty.into_common_response().to_json())
}
