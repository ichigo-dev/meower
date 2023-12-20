//------------------------------------------------------------------------------
//! WorkspaceMemberAuthority model.
//------------------------------------------------------------------------------

use crate::Validate;

use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Map.
//------------------------------------------------------------------------------
#[derive(Copy, Clone)]
pub enum AuthorityMap
{
    Admin = 100,
    General = 50,
    ReadOnly = 10,
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "workspace_member_authority")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub workspace_member_authority_id: i64,
    pub symbol: String,
    pub value: i32,
}

impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds workspace_member_authority by authority.
    //--------------------------------------------------------------------------
    pub fn find_by_authority( authority: &AuthorityMap ) -> Select<Self>
    {
        Self::find().filter(Column::Value.eq(*authority as i32))
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
impl ActiveModelBehavior for ActiveModel {}

impl Validate for ActiveModel {}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMember,
}

impl Related<super::workspace_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::WorkspaceMember.def()
    }
}
