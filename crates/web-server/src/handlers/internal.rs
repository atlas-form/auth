use std::sync::Arc;

use axum::Extension;
use toolcraft_axum_kit::{IntoCommonResponse, ResponseResult};

use crate::{dto::internal::JwtVerifyConfigResponse, settings::JwtVerifyConfig};

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
