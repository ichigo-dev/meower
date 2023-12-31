//------------------------------------------------------------------------------
//! OrganizationMember model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;

use sea_orm::entity::prelude::*;
use rust_i18n::t;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("OrganizationMember: Database error.")]
    DbError(#[from] DbErr),
}

impl Error
{
    //--------------------------------------------------------------------------
    /// Gets the error message.
    //--------------------------------------------------------------------------
    pub fn get_error_message( &self ) -> (Option<Column>, String)
    {
        match self
        {
            Self::DbError(_) => (None, t!("common.error.db")),
        }
    }
}


//------------------------------------------------------------------------------
/// Column.
//------------------------------------------------------------------------------
impl Column
{
    //--------------------------------------------------------------------------
    /// Gets the column name.
    //--------------------------------------------------------------------------
    pub fn get_name( &self ) -> String
    {
        match self
        {
            Self::OrganizationMemberId => t!("entities.organization_member.organization_member_id.name"),
            Self::OrganizationId => t!("entities.organization_member.organization_id.name"),
            Self::UserAccountId => t!("entities.organization_member.user_account_id.name"),
            Self::OrganizationMemberAuthorityId => t!("entities.organization_member.organization_member_authority_id.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organization_member")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub organization_member_id: i64,
    pub organization_id: i64,
    pub user_account_id: i64,
    pub organization_member_authority_id: i64,
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
impl ActiveModelBehavior for ActiveModel {}
impl ValidateExt for ActiveModel { type Error = Error; }


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
        belongs_to = "super::organization_member_authority::Entity",
        from = "Column::OrganizationMemberAuthorityId",
        to = "super::organization_member_authority::Column::OrganizationMemberAuthorityId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    OrganizationMemberAuthority,

    #[sea_orm(
        belongs_to = "super::user_account::Entity",
        from = "Column::UserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,
}

impl Related<super::organization::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Organization.def()
    }
}

impl Related<super::organization_member_authority::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::OrganizationMemberAuthority.def()
    }
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}
