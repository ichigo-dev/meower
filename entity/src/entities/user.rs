//------------------------------------------------------------------------------
//! User table model.
//------------------------------------------------------------------------------

use meower_utility::{ Auth, Validator, Config };
use sea_orm::entity::prelude::*;
use sea_orm::{ ConnectionTrait, ActiveValue, ActiveModelTrait };
use sea_query::Condition;
use async_trait::async_trait;


//------------------------------------------------------------------------------
/// User table model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub account_name: String,
    pub password: String,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Logs in the user.
    //--------------------------------------------------------------------------
    pub async fn login
    (
        hdb: &DbConn,
        email: &str,
        password: &str,
    ) -> bool
    {
        match Self::find()
            .filter(Column::Email.contains(email))
            .one(hdb)
            .await
            .unwrap()
        {
            Some(user) =>
            {
                return Auth::password_verify(password, &user.password);
            },
            None =>
            {
                return false;
            },
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Creates a new user.
    //--------------------------------------------------------------------------
    fn new() -> Self
    {
        Self
        {
            id: ActiveValue::NotSet,
            ..ActiveModelTrait::default()
        }
    }

    //--------------------------------------------------------------------------
    /// Will be called before saving.
    //--------------------------------------------------------------------------
    async fn before_save<C>
    (
        mut self,
        hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let account_name = self.account_name.clone().unwrap();
        let password = self.password.clone().unwrap();
        let email = self.email.clone().unwrap();

        // Check if the account already exists
        if insert
        {
            let exists_user = Entity::find()
                .filter
                (
                    Condition::any()
                        .add(Column::AccountName.eq(account_name.clone()))
                        .add(Column::Email.eq(email.clone()))
                )
                .one(hdb)
                .await
                .unwrap();
            if exists_user.is_some()
            {
                return Err(DbErr::Custom("The account already exists.".to_string()));
            }
        }

        // Validates account name.
        let mut account_name_validator = Validator::new(&account_name)
            .not_empty("Account name is required.")
            .min_len(3, "Account name is too short.")
            .max_len(255, "Account name is too long.")
            .regex(r"^[a-zA-Z0-9_]+$", "Account name must contain only letters, numbers, and underscores.")
            .validate();
        if account_name_validator.validate() == false
        {
            let errors = account_name_validator.errors();
            return Err(DbErr::Custom(errors[0].to_string()));
        }

        // Validates email.
        let mut email_validator = Validator::new(&email)
            .not_empty("Email is empty.")
            .max_len(255, "Email is too long.")
            .is_email("Email is invalid.")
            .validate();
        if email_validator.validate() == false
        {
            let errors = account_name_validator.errors();
            return Err(DbErr::Custom(errors[0].to_string()));
        }

        // Validates password.
        let mut password_validator = Validator::new(&password)
            .not_empty("Password is empty.")
            .min_len(8, "Password is too short.")
            .max_len(255, "Password is too long.")
            .regex(r".*[a-zA-Z].*", "Password must contain at least one letter.")
            .regex(r".*[0-9].*", "Password must contain at least one number.")
            .regex(r".*[!@#$%^&*()].*", "Password must contain at least one special character.")
            .validate();
        if password_validator.validate() == false
        {
            let errors = account_name_validator.errors();
            return Err(DbErr::Custom(errors[0].to_string()));
        }

        // Hashes the password.
        let config = Config::init();
        let hash = Auth::password_hash(&password, &config);
        self.set(Column::Password, hash.into());

        Ok(self)
    }
}
