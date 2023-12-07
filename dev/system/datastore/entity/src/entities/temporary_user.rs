//------------------------------------------------------------------------------
//! TemporaryUser model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;


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
