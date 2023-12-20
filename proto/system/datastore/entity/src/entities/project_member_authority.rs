//------------------------------------------------------------------------------
//! ProjectMemberAuthority model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "project_member_authority")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub project_member_authority_id: i64,
    pub symbol: String,
    pub value: i32,
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
    #[sea_orm(has_many = "super::project_member::Entity")]
    ProjectMember,
}

impl Related<super::project_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ProjectMember.def()
    }
}
