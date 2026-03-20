use db_core::{DbContext, Error, Repository, Result};
use sea_orm::*;
use time::OffsetDateTime;
use uuid::Uuid;

use super::dto::{CreateUser, UpdateUser, User};
use crate::entity::users;

db_core::impl_repository!(UsersRepo, users::Entity, users::Model);

pub struct UsersService {
    repo: UsersRepo,
}

impl UsersService {
    pub fn new(db: DbContext) -> Self {
        Self {
            repo: UsersRepo::new(db),
        }
    }

    pub async fn create(&self, input: CreateUser) -> Result<User> {
        let now = OffsetDateTime::now_utc();
        let (id, display_user_id) = self.generate_unique_display_user_id().await?;
        let display_name = input.display_name.unwrap_or_else(|| input.username.clone());
        let model = users::ActiveModel {
            id: Set(id),
            display_user_id: Set(Some(display_user_id)),
            username: Set(input.username),
            display_name: Set(Some(display_name)),
            avatar: Set(input.avatar),
            password: Set(input.password),
            email: Set(input.email),
            email_verified: Set(false),
            disabled: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
        };
        let result = self.repo.insert(model).await?;
        Ok(Self::from_model(result))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let model = self.repo.find_by_id(id.to_owned()).await?;
        Ok(model.map(Self::from_model))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let query = self
            .repo
            .query()
            .filter(users::Column::Username.eq(username));
        let model = self.repo.select_one(query).await?;
        Ok(model.map(Self::from_model))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let query = self.repo.query().filter(users::Column::Email.eq(email));
        let model = self.repo.select_one(query).await?;
        Ok(model.map(Self::from_model))
    }

    pub async fn find_by_display_user_id(&self, display_user_id: &str) -> Result<Option<User>> {
        let query = self
            .repo
            .query()
            .filter(users::Column::DisplayUserId.eq(display_user_id));
        let model = self.repo.select_one(query).await?;
        Ok(model.map(Self::from_model))
    }

    pub async fn update(&self, id: &str, input: UpdateUser) -> Result<User> {
        let existing = self
            .repo
            .find_by_id(id.to_owned())
            .await?
            .ok_or_else(|| Error::not_found("User", id))?;

        let mut model: users::ActiveModel = existing.into();

        if let Some(display_name) = input.display_name {
            model.display_name = Set(display_name);
        }
        if let Some(avatar) = input.avatar {
            model.avatar = Set(avatar);
        }
        if let Some(password) = input.password {
            model.password = Set(password);
        }
        if let Some(email) = input.email {
            model.email = Set(email);
        }
        if let Some(email_verified) = input.email_verified {
            model.email_verified = Set(email_verified);
        }
        if let Some(disabled) = input.disabled {
            model.disabled = Set(disabled);
        }
        model.updated_at = Set(OffsetDateTime::now_utc());

        let result = self.repo.update(model).await?;
        Ok(Self::from_model(result))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.repo.delete_by_id(id.to_owned()).await?;
        Ok(())
    }

    fn from_model(model: users::Model) -> User {
        User {
            id: model.id,
            display_user_id: model.display_user_id,
            username: model.username,
            display_name: model.display_name,
            avatar: model.avatar,
            password: model.password,
            email: model.email,
            email_verified: model.email_verified,
            disabled: model.disabled,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }

    async fn generate_unique_display_user_id(&self) -> Result<(String, String)> {
        for _ in 0 .. 16 {
            let uuid = Uuid::new_v4();
            let display_user_id = short_id_from_uuid(uuid);
            if self
                .find_by_display_user_id(&display_user_id)
                .await?
                .is_none()
            {
                return Ok((uuid.to_string(), display_user_id));
            }
        }
        Err(Error::internal(
            "failed to generate unique display_user_id after retries",
        ))
    }
}

fn short_id_from_uuid(uuid: Uuid) -> String {
    let mut hash: u64 = 1469598103934665603;
    for b in uuid.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{:012}", hash % 1_000_000_000_000)
}
