//------------------------------------------------------------------------------
//! UserAuth model.
//------------------------------------------------------------------------------

use meower_core::{ Config, Validator, I18n };
use crate::Validate;

use argon2::{ Argon2, PasswordHash, PasswordHasher, PasswordVerifier };
use argon2::password_hash::SaltString;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_auth")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_auth_id: i64,
    pub user_id: i64,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Hashes password.
    //--------------------------------------------------------------------------
    pub fn password_hash( password: &str ) -> String
    {
        let config = Config::new();
        let bin_password = password.as_bytes();
        let salt = SaltString::from_b64(config.get("argon2.phc_salt").as_ref())
            .unwrap();
        let argon2 = Argon2::new
        (
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::default(),
        );
        argon2.hash_password(bin_password, &salt)
            .unwrap()
            .to_string()
    }

    //--------------------------------------------------------------------------
    /// Verifies password.
    //--------------------------------------------------------------------------
    pub fn password_verify( &self, password: &str ) -> bool
    {
        let parsed_hash = PasswordHash::new(&self.password).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
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
        let hash = Model::password_hash(&password);
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
        self.set(Column::UpdatedAt, now.into());

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
        _hdb: &C,
        i18n: &I18n,
    ) -> Result<(), String>
    where
        C: ConnectionTrait,
    {
        let password = self.password.clone().unwrap();

        // Validates fields.
        let mut password_validator = Validator::new(&password)
            .not_empty(&i18n.get("model_user_auth.error.password.not_empty"))
            .min_len
            (
                8,
                &i18n.get_with
                (
                    "model_user_auth.error.password.min_len",
                    [("min_len", "8")].into()
                )
            )
            .max_len
            (
                255,
                &i18n.get_with
                (
                    "model_user_auth.error.password.max_len",
                    [("max_len", "255")].into()
                )
            )
            .regex
            (
                r".*[a-zA-Z].*",
                &i18n.get("model_user_auth.error.password.regex")
            )
            .regex
            (
                r".*[0-9].*",
                &i18n.get("model_user_auth.error.password.regex")
            )
            .regex
            (
                r".*[!@#$%^&*()].*",
                &i18n.get("model_user_auth.error.password.regex")
            )
            .validate();
        if password_validator.has_err()
        {
            return Err(password_validator.get_first_error());
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
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::User.def()
    }
}
