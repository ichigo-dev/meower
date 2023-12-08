//------------------------------------------------------------------------------
//! TemporaryUser model.
//------------------------------------------------------------------------------

use meower_core::{ I18n, Config, Mailer };
use crate::Validate;
use super::user::ActiveModel as ActiveUser;
use super::user_auth::Model as UserAuthModel;
use super::user_auth::ActiveModel as ActiveUserAuth;
use super::user_account::ActiveModel as ActiveUserAccount;
use super::temporary_user_token::Entity as TemporaryUserTokenEntity;

use argon2::PasswordHash;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "temporary_user")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub temporary_user_id: i64,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub user_account_name: String,
    pub created_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Finds temporary_user by email.
    //--------------------------------------------------------------------------
    pub async fn find_by_email<C>( hdb: &C, email: &str ) -> Option<Self>
    where
        C: ConnectionTrait,
    {
        let user = Entity::find()
            .filter(Column::Email.contains(email))
            .one(hdb)
            .await
            .unwrap_or(None);
        user
    }

    //--------------------------------------------------------------------------
    /// Sends a signup email.
    //--------------------------------------------------------------------------
    pub async fn send_signup_email<C>
    (
        &self,
        hdb: &C,
        config: &Config,
        i18n: &I18n,
    ) -> Result<(), String>
    where
        C: ConnectionTrait,
    {
        let temporary_user_token = match self
            .find_related(TemporaryUserTokenEntity)
            .one(hdb)
            .await
            .unwrap()
        {
            Some(temporary_user_token) => temporary_user_token,
            None =>
            {
                return Err
                (
                    i18n.get("model_temporary_user.error.token_not_found")
                );
            },
        };
        let message = Mailer::message()
            .from(config.get("email_from").parse().unwrap())
            .to(self.email.clone().parse().unwrap())
            .subject("Signup")
            .body(format!("Signup: {}", temporary_user_token.token))
            .unwrap();
        let mailer = Mailer::new(&config);
        match mailer.send(message).await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
impl ActiveModel
{
    //--------------------------------------------------------------------------
    /// Hashes password.
    //--------------------------------------------------------------------------
    pub fn hash_password( mut self ) -> Self
    {
        // Hashes the password.
        let password = self.password.clone().unwrap();
        let hash = UserAuthModel::password_hash(&password);
        self.set(Column::Password, hash.into());
        self
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Before save.
    //--------------------------------------------------------------------------
    async fn before_save<C>
    (
        mut self,
        _hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());
        }

        // Hashes the password.
        let password = self.password.clone().unwrap();
        if let Err(_) = PasswordHash::new(&password)
        {
            self = self.hash_password();
        };

        Ok(self)
    }
}

#[async_trait]
impl Validate for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Validates the data.
    //--------------------------------------------------------------------------
    async fn validate<C>
    (
        &self,
        hdb: &C,
        i18n: &I18n,
    ) -> Result<(), String>
    where
        C: ConnectionTrait,
    {
        let email = self.email.clone().unwrap();
        let password = self.password.clone().unwrap();
        let user_account_name = self.user_account_name.clone().unwrap();

        // Checks if the email already exists.
        if Model::find_by_email(hdb, &email).await.is_some()
        {
            return Err
            (
                i18n.get("model_temporary_user.error.email.already_exists")
            );
        }

        // User validation.
        let user = ActiveUser
        {
            email: ActiveValue::Set(email),
            ..Default::default()
        };
        if let Err(e) = user.validate(hdb, &i18n).await
        {
            return Err(e.to_string());
        };

        // UserAuth validation.
        let user_auth = ActiveUserAuth
        {
            password: ActiveValue::Set(password),
            ..Default::default()
        };
        if let Err(e) = user_auth.validate(hdb, &i18n).await
        {
            return Err(e.to_string());
        };

        // UserAccount validation.
        let user_account = ActiveUserAccount
        {
            user_account_name: ActiveValue::Set(user_account_name),
            ..Default::default()
        };
        if let Err(e) = user_account.validate(hdb, &i18n).await
        {
            return Err(e.to_string());
        };

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::temporary_user_token::Entity")]
    TemporaryUserToken,
}

impl Related<super::temporary_user_token::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::TemporaryUserToken.def()
    }
}
