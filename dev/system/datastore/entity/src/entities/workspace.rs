//------------------------------------------------------------------------------
//! Workspace model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "workspace")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub workspace_id: i64,
    #[sea_orm(unique)]
    pub workspace_name: String,
    pub display_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
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
    #[sea_orm(has_many = "super::project::Entity")]
    Project,

    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMember,

    #[sea_orm(has_many = "super::user_account_workspace::Entity")]
    UserAccountWorkspace,

    #[sea_orm(has_many = "super::organization_workspace::Entity")]
    OrganizationWorkspace,
}

impl Related<super::project::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Project.def()
    }
}

impl Related<super::workspace_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::WorkspaceMember.def()
    }
}

impl Related<super::user_account_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccountWorkspace.def()
    }
}

impl Related<super::organization_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::OrganizationWorkspace.def()
    }
}
