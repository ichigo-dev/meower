//------------------------------------------------------------------------------
//! User model.
//------------------------------------------------------------------------------

use meower_core::Validator;
use super::user_auth::Entity as UserAuthEntity;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;


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

impl Model
{
    //--------------------------------------------------------------------------
    /// Finds user by email.
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
    /// Tries to login.
    //--------------------------------------------------------------------------
    pub async fn try_login( &self, hdb: &DbConn, password: &str ) -> bool
    {
        if let Some(user_auth) = self
            .find_related(UserAuthEntity)
            .one(hdb)
            .await
            .unwrap_or(None)
        {
            return user_auth.password_verify(password);
        }
        false
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
        hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let email = self.email.clone().unwrap();

        // Checks if the account already exists.
        if insert
        {
            if Model::find_by_email(hdb, &email).await.is_some()
            {
                let error = "model_user.error.email.already_exists".to_string();
                return Err(DbErr::Custom(error));
            }
        }

        // Validates fields.
        let mut email_validator = Validator::new(&email)
            .not_empty("model_user.error.email.not_empty")
            .max_len(255, "model_user.error.email.max_len")
            .is_email("model_user.error.email.invalid")
            .validate();
        if email_validator.has_err()
        {
            return Err(DbErr::Custom(email_validator.get_first_error()));
        }

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
