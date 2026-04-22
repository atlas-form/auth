use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ── Requests ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 1, max = 64))]
    pub display_name: Option<String>,
    #[validate(url)]
    pub avatar: Option<String>,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 128))]
    pub identifier: String,
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePasswordRequest {
    #[serde(rename = "oldPassword")]
    #[validate(length(min = 1, max = 128))]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateEmailRequest {
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileRequest {
    #[serde(default, deserialize_with = "deserialize_optional_nullable_string")]
    pub display_name: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_optional_nullable_string")]
    pub avatar: Option<Option<String>>,
}

fn deserialize_optional_nullable_string<'de, D>(
    deserializer: D,
) -> Result<Option<Option<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<Option<String>>::deserialize(deserializer)
}

// ── Responses ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub display_user_id: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub disabled: bool,
}
