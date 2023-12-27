//------------------------------------------------------------------------------
//! Project model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "project")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub project_id: i64,
    pub project_name: String,
    pub display_name: String,
    pub workspace_id: i64,
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
    #[sea_orm(has_many = "super::project_member::Entity")]
    ProjectMember,

    #[sea_orm(has_many = "super::task::Entity")]
    Task,

    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::WorkspaceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Workspace,
}

impl Related<super::project_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ProjectMember.def()
    }
}

impl Related<super::task::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Task.def()
    }
}

impl Related<super::workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Workspace.def()
    }
}