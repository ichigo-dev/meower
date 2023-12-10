//------------------------------------------------------------------------------
//! User model.
//------------------------------------------------------------------------------

use meower_core::{ Validator, I18n, Mailer, Config, mail_header };
use crate::{ Validate, FieldVerify };
use super::user_auth::Entity as UserAuthEntity;
use super::reset_password_token::ActiveModel as ActiveResetPasswordToken;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_id: i64,
    #[sea_orm(unique)]
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds user by email.
    //--------------------------------------------------------------------------
    pub fn find_by_email( email: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Email.eq(email))
    }
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Tries to login.
    //--------------------------------------------------------------------------
    pub async fn try_login<C>( &self, hdb: &C, password: &str ) -> bool
    where
        C: ConnectionTrait,
    {
        if let Some(user_auth) = self
            .find_related(UserAuthEntity)
            .one(hdb)
            .await
            .unwrap_or(None)
        {
            return user_auth.verify_field(password);
        }
        false
    }

    //--------------------------------------------------------------------------
    /// Sends a reset password mail.
    //--------------------------------------------------------------------------
    pub async fn send_reset_password_mail<C>
    (
        &self,
        hdb: &C,
        config: &Config,
        i18n: &I18n,
    ) -> Result<(), String>
    where
        C: ConnectionTrait,
    {
        let reset_password_token = ActiveResetPasswordToken
        {
            user_id: ActiveValue::Set(self.user_id),
            ..Default::default()
        };
        let reset_password_token = match reset_password_token.insert(hdb).await
        {
            Ok(reset_password_token) => reset_password_token,
            Err(e) => return Err(e.to_string()),
        };

        let reset_password_url = format!
        (
            "{}/auth/reset_password/{}",
            config.get("system.auth_server_url"),
            reset_password_token.token,
        );
        let template = Mailer::get_template_with
        (
            "auth_server/reset_password.html",
            &config,
            &i18n,
            [
                ("reset_password_url", reset_password_url.as_str())
            ].into(),
        );
        let message = Mailer::message()
            .from(config.get("email.from").parse().unwrap())
            .to(self.email.clone().parse().unwrap())
            .subject(i18n.get("model_user.reset_password_mail.subject"))
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
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
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
            self.set(Column::IsDeleted, false.into());
        }
        self.set(Column::UpdatedAt, now.into());

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

        // Checks if the account already exists.
        if self.user_id.is_set() == false
        {
            if Entity::find_by_email(&email).one(hdb).await.unwrap().is_some()
            {
                return Err(i18n.get("model_user.error.email.already_exists"));
            }
        }

        // Validates fields.
        let mut email_validator = Validator::new(&email)
            .not_empty(&i18n.get("model_user.error.email.not_empty"))
            .max_len
            (
                255,
                &i18n.get_with
                (
                    "model_user.error.email.max_len",
                    [("max_len", "255")].into()
                )
            )
            .is_email(&i18n.get("model_user.error.email.invalid"))
            .validate();
        if email_validator.has_err()
        {
            return Err(email_validator.get_first_error());
        }

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::user_account::Entity")]
    UserAccount,

    #[sea_orm(has_one = "super::user_auth::Entity")]
    UserAuth,

    #[sea_orm(has_one = "super::reset_password_token::Entity")]
    ResetPasswordToken,

    #[sea_orm(has_one = "super::user_jwt_subject::Entity")]
    UserJwtSubject,
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}

impl Related<super::user_auth::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAuth.def()
    }
}

impl Related<super::reset_password_token::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ResetPasswordToken.def()
    }
}

impl Related<super::user_jwt_subject::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserJwtSubject.def()
    }
}
