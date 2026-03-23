use std::sync::Arc;

use axum::{
    Extension,
    extract::Path,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use service::api::auth::AuthApi;
use toolcraft_axum_kit::{ApiError, IntoCommonResponse, ResponseResult};

use crate::{
    dto::internal::{DisplayUserIdToUuidResponse, JwtVerifyConfigResponse},
    error::Error,
    settings::JwtVerifyConfig,
    statics::{db_manager::get_default_ctx, internal_auth::get_internal_auth_config},
};

fn svc(e: db_core::Error) -> ApiError {
    ApiError::from(Error::from(e))
}

pub async fn jwt_verify_config(
    Extension(cfg): Extension<Arc<JwtVerifyConfig>>,
) -> ResponseResult<JwtVerifyConfigResponse> {
    Ok(JwtVerifyConfigResponse {
        public_key_pem: cfg.public_key_pem.clone(),
        issuer: cfg.issuer.clone(),
        audience: cfg.audience.clone(),
    }
    .into_common_response()
    .to_json())
}

pub async fn display_user_id_to_uuid(
    Path(display_user_id): Path<String>,
) -> ResponseResult<DisplayUserIdToUuidResponse> {
    let api = AuthApi::new(get_default_ctx());
    let user = api
        .get_user_by_display_user_id(&display_user_id)
        .await
        .map_err(svc)?;

    Ok(DisplayUserIdToUuidResponse { id: user.id }
        .into_common_response()
        .to_json())
}

pub async fn internal_auth(req: Request<axum::body::Body>, next: Next) -> Response {
    let cfg = get_internal_auth_config();
    let token = req
        .headers()
        .get(&cfg.header_name)
        .and_then(|v| v.to_str().ok());

    if token != Some(cfg.token.as_str()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(req).await
}
