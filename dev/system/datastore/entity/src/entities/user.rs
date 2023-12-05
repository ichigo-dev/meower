//------------------------------------------------------------------------------
//! User model.
//------------------------------------------------------------------------------

use super::user_auth::Entity as UserAuthEntity;
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
    pub async fn find_by_email( hdb: &DbConn, email: &str ) -> Option<Self>
    {
        let user = Entity::find()
            .filter(Column::Email.contains(email))
            .one(hdb)
            .await
            .unwrap();
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
            .unwrap()
        {
            return user_auth.verify(password);
        }
        false
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
impl ActiveModelBehavior for ActiveModel {}


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
