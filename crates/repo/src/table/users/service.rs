use sea_orm::*;
use time::OffsetDateTime;
use uuid::Uuid;

use db_core::{DbContext, Error, Repository, Result};

use crate::entity::users;

use super::dto::{CreateUser, UpdateUser, User};

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
        let model = users::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            username: Set(input.username),
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
        let query = self
            .repo
            .query()
            .filter(users::Column::Email.eq(email));
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
            username: model.username,
            password: model.password,
            email: model.email,
            email_verified: model.email_verified,
            disabled: model.disabled,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
