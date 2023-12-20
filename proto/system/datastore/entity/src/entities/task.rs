//------------------------------------------------------------------------------
//! Task model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub task_id: i64,
    pub project_id: i64,
    pub owner_user_account_id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
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
    #[sea_orm(
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::ProjectId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Project,

    #[sea_orm(
        belongs_to = "super::user_account::Entity",
        from = "Column::OwnerUserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,

    #[sea_orm(has_many = "super::task_member::Entity")]
    TaskMember,
}

impl Related<super::project::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Project.def()
    }
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}

impl Related<super::task_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::TaskMember.def()
    }
}
