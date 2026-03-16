use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use repo::db_core::{DbContext, Error, Result};
use repo::table::users::{UpdateUser, UsersService};

use crate::dto::auth::{AuthUser, LoginRequest, RegisterRequest, UpdatePasswordRequest};

pub struct AuthApi {
    users_svc: UsersService,
}

impl AuthApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            users_svc: UsersService::new(db),
        }
    }

    /// 注册新用户（校验 username/email 唯一性，哈希密码）
    pub async fn register(&self, req: RegisterRequest) -> Result<AuthUser> {
        // 检查 username 是否已存在
        if self.users_svc.find_by_username(&req.username).await?.is_some() {
            return Err(Error::already_exists("User", "username", &req.username));
        }

        // 检查 email 是否已存在
        if let Some(ref email) = req.email {
            if self.users_svc.find_by_email(email).await?.is_some() {
                return Err(Error::already_exists("User", "email", email));
            }
        }

        let hashed = hash_password(&req.password)?;

        let user = self
            .users_svc
            .create(repo::table::users::CreateUser {
                username: req.username,
                password: hashed,
                email: req.email,
            })
            .await?;

        Ok(into_auth_user(user))
    }

    /// 登录（identifier 可以是 username 或 email）
    pub async fn login(&self, req: LoginRequest) -> Result<AuthUser> {
        let user = self
            .users_svc
            .find_by_username(&req.identifier)
            .await?
            .or(
                // 尝试用 email 查找
                self.users_svc.find_by_email(&req.identifier).await?,
            )
            .ok_or_else(|| Error::not_found("User", &req.identifier))?;

        if user.disabled {
            return Err(Error::permission_denied("User account is disabled"));
        }

        verify_password(&req.password, &user.password)?;

        Ok(into_auth_user(user))
    }

    /// 获取用户信息
    pub async fn get_user(&self, id: &str) -> Result<AuthUser> {
        let user = self
            .users_svc
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("User", id))?;

        Ok(into_auth_user(user))
    }

    /// 修改密码（需要验证旧密码）
    pub async fn update_password(&self, id: &str, req: UpdatePasswordRequest) -> Result<()> {
        let user = self
            .users_svc
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("User", id))?;

        verify_password(&req.old_password, &user.password)?;

        let hashed = hash_password(&req.new_password)?;
        self.users_svc
            .update(
                id,
                UpdateUser {
                    password: Some(hashed),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    /// 修改邮箱（清空 email_verified）
    pub async fn update_email(&self, id: &str, email: Option<String>) -> Result<()> {
        // 检查新 email 是否被其他用户占用
        if let Some(ref new_email) = email {
            if let Some(existing) = self.users_svc.find_by_email(new_email).await? {
                if existing.id != id {
                    return Err(Error::already_exists("User", "email", new_email));
                }
            }
        }

        self.users_svc
            .update(
                id,
                UpdateUser {
                    email: Some(email),
                    email_verified: Some(false),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    /// 标记邮箱已验证
    pub async fn verify_email(&self, id: &str) -> Result<()> {
        self.users_svc
            .update(
                id,
                UpdateUser {
                    email_verified: Some(true),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    /// 启用/禁用用户
    pub async fn set_disabled(&self, id: &str, disabled: bool) -> Result<()> {
        self.users_svc
            .update(
                id,
                UpdateUser {
                    disabled: Some(disabled),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    /// 删除用户
    pub async fn delete_user(&self, id: &str) -> Result<()> {
        // 确认用户存在
        self.users_svc
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::not_found("User", id))?;

        self.users_svc.delete(id).await
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| Error::internal(e.to_string()))?
        .to_string();
    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> Result<()> {
    let parsed = PasswordHash::new(hash).map_err(|e| Error::internal(e.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| Error::permission_denied("Invalid password"))
}

fn into_auth_user(user: repo::table::users::User) -> AuthUser {
    AuthUser {
        id: user.id,
        username: user.username,
        email: user.email,
        email_verified: user.email_verified,
        disabled: user.disabled,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}
