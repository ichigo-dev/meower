//------------------------------------------------------------------------------
//! UserAuth model.
//------------------------------------------------------------------------------

use meower_core::{ Config, Validator };

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
        let config = Config::get();
        let bin_password = password.as_bytes();
        let salt = SaltString::from_b64(config.argon2_phc_salt().as_ref())
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
        let password = self.password.clone().unwrap();

        // Validates fields.
        let mut password_validator = Validator::new(&password)
            .not_empty("model_user_auth.error.password.not_empty")
            .min_len(8, "model_user_auth.error.password.min_len")
            .max_len(255, "model_user_auth.error.password.max_len")
            .regex(r".*[a-zA-Z].*", "model_user_auth.error.password.regex")
            .regex(r".*[0-9].*", "model_user_auth.error.password.regex")
            .regex(r".*[!@#$%^&*()].*", "model_user_auth.error.password.regex")
            .validate();
        if password_validator.has_err()
        {
            return Err(DbErr::Custom(password_validator.get_first_error()));
        }

        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());
        }
        self.set(Column::UpdatedAt, now.into());

        // Hashes the password.
        let hash = Model::password_hash(&password);
        self.set(Column::Password, hash.into());

        Ok(self)
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
