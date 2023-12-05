//------------------------------------------------------------------------------
//! ProjectMember model.
//------------------------------------------------------------------------------

use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "project_member")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub project_member_id: i64,
    pub project_id: i64,
    pub user_account_id: i64,
    pub project_member_authority_id: i64,
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
        belongs_to = "super::project_member_authority::Entity",
        from = "Column::ProjectMemberAuthorityId",
        to = "super::project_member_authority::Column::ProjectMemberAuthorityId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProjectMemberAuthority,

    #[sea_orm(
        belongs_to = "super::user_account::Entity",
        from = "Column::UserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,
}

impl Related<super::project::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Project.def()
    }
}

impl Related<super::project_member_authority::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ProjectMemberAuthority.def()
    }
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}
