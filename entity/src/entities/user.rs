//------------------------------------------------------------------------------
//! User table model.
//------------------------------------------------------------------------------

use meower_utility::Auth;
use sea_orm::entity::prelude::*;
use sea_orm::{ ConnectionTrait, ActiveValue, ActiveModelTrait };
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
        self,
        _db: &C,
        _insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}
