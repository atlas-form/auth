use time::OffsetDateTime;

/// 完整用户数据，对应 users 表
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub disabled: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// 创建用户参数（id 和时间戳由 Service 生成）
#[derive(Debug, Clone)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

/// 更新用户参数（None 表示不修改该字段）
#[derive(Debug, Clone, Default)]
pub struct UpdateUser {
    pub password: Option<String>,
    /// None = 不修改；Some(None) = 清空；Some(Some(v)) = 设置新值
    pub email: Option<Option<String>>,
    pub email_verified: Option<bool>,
    pub disabled: Option<bool>,
}
