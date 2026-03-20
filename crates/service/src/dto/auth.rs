use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// 注册请求
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
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

/// 修改基础资料请求
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProfileRequest {
    /// None = 不修改；Some(None) = 清空；Some(Some(v)) = 更新
    pub display_name: Option<Option<String>>,
    /// None = 不修改；Some(None) = 清空；Some(Some(v)) = 更新
    pub avatar: Option<Option<String>>,
}

/// Auth 层对外暴露的用户结构（不含密码）
#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub id: String,
    pub display_user_id: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub disabled: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
