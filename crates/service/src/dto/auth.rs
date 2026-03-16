use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 注册请求
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

/// 登录请求（identifier 可以是 username 或 email）
#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

/// 修改密码请求
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

/// Auth 层对外暴露的用户结构（不含密码）
#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub disabled: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
