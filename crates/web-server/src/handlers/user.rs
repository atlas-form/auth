use std::time::{SystemTime, UNIX_EPOCH};

use axum::{Extension, Json};
use service::api::auth::AuthApi;
use toolcraft_axum_kit::{
    ApiError, CommonOk, Empty, IntoCommonResponse, ResponseResult, middleware::auth_mw::AuthUser,
};
use validator::Validate;

use crate::{
    dto::auth::{UpdateEmailRequest, UpdatePasswordRequest, UpdateProfileRequest, UserResponse},
    error::{Error, from_biz_error},
    settings::AvatarUrlConfig,
    statics::db_manager::get_default_ctx,
};

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
pub async fn me(
    Extension(auth_user): Extension<AuthUser>,
    Extension(avatar_cfg): Extension<std::sync::Arc<AvatarUrlConfig>>,
) -> ResponseResult<UserResponse> {
    let api = AuthApi::new(get_default_ctx());
    let user = api
        .get_user(&auth_user.user_id)
        .await
        .map_err(from_biz_error)?;
    let avatar = build_avatar_response(user.avatar.as_deref(), &avatar_cfg);

    Ok(UserResponse {
        display_user_id: user.display_user_id,
        username: user.username,
        display_name: user.display_name,
        avatar,
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
    .map_err(from_biz_error)?;

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
        .map_err(from_biz_error)?;

    Ok(Empty.into_common_response().to_json())
}

#[utoipa::path(
    put,
    path = "/profile",
    tag = "User",
    security(("Bearer" = [])),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = CommonOk),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn update_profile(
    Extension(auth_user): Extension<AuthUser>,
    Extension(avatar_cfg): Extension<std::sync::Arc<AvatarUrlConfig>>,
    Json(req): Json<UpdateProfileRequest>,
) -> ResponseResult<Empty> {
    let req = normalize_profile_patch(req, &avatar_cfg).map_err(ApiError::from)?;

    let api = AuthApi::new(get_default_ctx());
    api.update_profile(
        &auth_user.user_id,
        service::dto::auth::UpdateProfileRequest {
            display_name: req.display_name,
            avatar: req.avatar,
        },
    )
    .await
    .map_err(from_biz_error)?;

    Ok(Empty.into_common_response().to_json())
}

fn normalize_profile_patch(
    req: UpdateProfileRequest,
    cfg: &AvatarUrlConfig,
) -> crate::error::Result<UpdateProfileRequest> {
    validate_profile_patch(&req, cfg)?;

    let avatar = match req.avatar {
        None => None,
        Some(None) => Some(None),
        Some(Some(avatar_key)) => Some(Some(with_avatar_version(&avatar_key)?)),
    };

    Ok(UpdateProfileRequest {
        display_name: req.display_name,
        avatar,
    })
}

fn validate_profile_patch(
    req: &UpdateProfileRequest,
    cfg: &AvatarUrlConfig,
) -> crate::error::Result<()> {
    if let Some(Some(display_name)) = &req.display_name
        && (display_name.is_empty() || display_name.chars().count() > 64) {
            return Err(Error::Custom(
                "display_name length must be between 1 and 64".to_string(),
            ));
        }

    if let Some(Some(avatar)) = &req.avatar {
        let key = avatar.trim();
        validate_avatar_key(key, cfg)?;
    }

    Ok(())
}

fn with_avatar_version(avatar_key: &str) -> crate::error::Result<String> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Custom(format!("failed to compute avatar version timestamp: {e}")))?
        .as_secs();
    let raw_key = avatar_key.trim();
    let raw_key = raw_key
        .rsplit_once("|v=")
        .map(|(key, _)| key)
        .unwrap_or(raw_key);
    let key = raw_key.split('?').next().unwrap_or_default().trim();
    Ok(format!("{key}|v={ts}"))
}

fn validate_avatar_key(key: &str, cfg: &AvatarUrlConfig) -> crate::error::Result<()> {
    if key.is_empty() || key.chars().count() > 512 {
        return Err(Error::Custom(
            "avatar key length must be between 1 and 512".to_string(),
        ));
    }

    if key.starts_with('/') {
        return Err(Error::Custom(
            "avatar must be object key only (must not start with '/')".to_string(),
        ));
    }

    if key.contains("://") || key.contains(':') {
        return Err(Error::Custom(
            "avatar must be object key only (URL is not allowed)".to_string(),
        ));
    }

    if key.chars().any(char::is_whitespace)
        || key.contains('?')
        || key.contains('#')
        || key.contains('&')
        || key.contains('=')
        || key.contains('|')
        || key.contains('\\')
    {
        return Err(Error::Custom("avatar key format is invalid".to_string()));
    }

    let endpoint = cfg.s3_endpoint.trim_end_matches('/');
    if key.starts_with(endpoint) {
        return Err(Error::Custom(
            "avatar must be object key only (endpoint prefix is not allowed)".to_string(),
        ));
    }

    Ok(())
}

fn parse_avatar_key_and_version(avatar: &str) -> Option<(String, Option<String>)> {
    let avatar = avatar.trim();
    if avatar.is_empty() {
        return None;
    }

    // legacy format: key?v=timestamp
    if let Some((key, query)) = avatar.split_once('?') {
        let key = key.trim();
        if key.is_empty() {
            return None;
        }
        return Some((key.to_string(), Some(query.to_string())));
    }

    // current storage format: key|v=timestamp
    if let Some((key, version_value)) = avatar.rsplit_once("|v=") {
        let key = key.trim();
        let version_value = version_value.trim();
        if !key.is_empty() && !version_value.is_empty() {
            return Some((key.to_string(), Some(format!("v={version_value}"))));
        }
    }

    Some((avatar.to_string(), None))
}

fn build_avatar_url(key: &str, version: Option<&str>, cfg: &AvatarUrlConfig) -> String {
    let key = key.trim_start_matches('/');
    let mut url = format!("{}/{}/{}", cfg.s3_endpoint, cfg.bucket, key);

    if let Some(version) = version.filter(|v| !v.is_empty()) {
        url.push('?');
        url.push_str(version);
    }

    url
}

fn build_avatar_response(avatar: Option<&str>, cfg: &AvatarUrlConfig) -> Option<String> {
    let avatar = avatar?.trim();
    if avatar.is_empty() {
        return None;
    }

    if avatar.starts_with("http://") || avatar.starts_with("https://") {
        return Some(avatar.to_string());
    }

    parse_avatar_key_and_version(avatar)
        .map(|(key, version)| build_avatar_url(&key, version.as_deref(), cfg))
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
    api.verify_email(&auth_user.user_id)
        .await
        .map_err(from_biz_error)?;

    Ok(Empty.into_common_response().to_json())
}
