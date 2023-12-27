//------------------------------------------------------------------------------
//! TemporaryUser model.
//------------------------------------------------------------------------------

use meower_core::{ I18n, Config, Mailer, mail_header };
use crate::{ Validate, FieldHash, FieldVerify, GenerateToken };
use super::user::Model as UserModel;
use super::user::ActiveModel as ActiveUser;
use super::user_auth::ActiveModel as ActiveUserAuth;
use super::temporary_user_code::{ self, Entity as TemporaryUserCodeEntity };

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
    pub token: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds temporary_user by email.
    //--------------------------------------------------------------------------
    pub fn find_by_email(  email: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Email.eq(email))
    }

    //--------------------------------------------------------------------------
    /// Finds temporary_user by token.
    //--------------------------------------------------------------------------
    pub fn find_by_token( token: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Token.eq(token))
    }
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Sends a verify mail.
    //--------------------------------------------------------------------------
    pub async fn send_verify_mail<C>
    (
        &self,
        hdb: &C,
        config: &Config,
        i18n: &I18n,
    ) -> Result<(), String>
    where
        C: ConnectionTrait,
    {
        let temporary_user_code = match self
            .find_related(TemporaryUserCodeEntity)
            .one(hdb)
            .await
            .unwrap()
        {
            Some(temporary_user_code) => temporary_user_code,
            None =>
            {
                return Err
                (
                    i18n.get("model_temporary_user.error.code_not_found")
                );
            },
        };

        let template = Mailer::get_template_with
        (
            "entrance/signup.html",
            &config,
            &i18n,
            [("verify_code", temporary_user_code.code.as_str())].into(),
        );
        let message = Mailer::message()
            .from(config.get("email.from").parse().unwrap())
            .to(self.email.clone().parse().unwrap())
            .subject(i18n.get("model_temporary_user.verify_mail.subject"))
            .header(mail_header::ContentType::TEXT_HTML)
            .body(template)
            .unwrap();
        let mailer = Mailer::new(&config);
        match mailer.send(message).await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    //--------------------------------------------------------------------------
    /// Creates a new user.
    //--------------------------------------------------------------------------
    pub async fn create_user<C>
    (
        &self,
        hdb: &C,
        i18n: &I18n,
    ) -> Result<UserModel, String>
    where
        C: ConnectionTrait,
    {
        // Creates a new user.
        let user = ActiveUser
        {
            email: ActiveValue::Set(self.email.clone()),
            ..Default::default()
        };
        let user = match user.validate_and_insert(hdb, &i18n).await
        {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        // Creates a new user_auth.
        let user_auth = ActiveUserAuth
        {
            user_id: ActiveValue::Set(user.user_id),
            password: ActiveValue::Set(self.password.clone()),
            ..Default::default()
        };
        if let Err(e) = user_auth.validate_and_insert(hdb, &i18n).await
        {
            return Err(e);
        };

        Ok(user)
    }
}

impl FieldVerify for Model
{
    //--------------------------------------------------------------------------
    /// Gets hashed field.
    //--------------------------------------------------------------------------
    fn get_hash_field( &self ) -> String
    {
        self.password.clone()
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
impl FieldHash for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Gets hashed field.
    //--------------------------------------------------------------------------
    fn get_hash_field( &self ) -> String
    {
        self.password.clone().unwrap()
    }

    //--------------------------------------------------------------------------
    /// Sets hashed field.
    //--------------------------------------------------------------------------
    fn set_hash_field( &mut self, hash: &str )
    {
        self.set(Column::Password, hash.to_string().into());
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
        if insert
        {
            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());

            let token = self.generate_token();
            self.set(Column::Token, token.into());
        }

        // Hashes the password.
        let password = self.password.clone().unwrap();
        if let Err(_) = PasswordHash::new(&password)
        {
            self.hash_field();
        };

        Ok(self)
    }

    //--------------------------------------------------------------------------
    /// Before delete.
    //--------------------------------------------------------------------------
    async fn before_delete<C>( mut self, hdb: &C ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let temporary_user_id = self.temporary_user_id.clone().unwrap();
        TemporaryUserCodeEntity::delete_many()
            .filter
            (
                temporary_user_code::Column::TemporaryUserId
                    .eq(temporary_user_id)
            )
            .exec(hdb)
            .await?;
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

        // Checks if the email already exists.
        if Entity::find_by_email(&email).one(hdb).await.unwrap().is_some()
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

        Ok(())
    }
}

impl GenerateToken for ActiveModel {}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::temporary_user_code::Entity")]
    TemporaryUserCode,
}

impl Related<super::temporary_user_code::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::TemporaryUserCode.def()
    }
}