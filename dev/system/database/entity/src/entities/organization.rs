//------------------------------------------------------------------------------
//! Organization model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;

use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("Organization: Database error.")]
    DbError(#[from] DbErr),
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organization")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub organization_id: i64,
    #[sea_orm(unique)]
    pub organization_name: String,
    pub display_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
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
    #[sea_orm(has_many = "super::organization_member::Entity")]
    OrganizationMember,

    #[sea_orm(has_many = "super::organization_workspace::Entity")]
    OrganizationWorkspace,
}

impl Related<super::organization_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::OrganizationMember.def()
    }
}

impl Related<super::organization_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::OrganizationWorkspace.def()
    }
}
