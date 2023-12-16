//------------------------------------------------------------------------------
//! OrganizationWorkspace model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "organization_workspace")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub organization_workspace_id: i64,
    pub organization_id: i64,
    pub workspace_id: i64,
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
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizationId",
        to = "super::organization::Column::OrganizationId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organization,

    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::WorkspaceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Workspace,
}

impl Related<super::organization::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Organization.def()
    }
}

impl Related<super::workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Workspace.def()
    }
}
