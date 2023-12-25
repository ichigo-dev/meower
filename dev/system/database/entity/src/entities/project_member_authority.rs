//------------------------------------------------------------------------------
//! ProjectMemberAuthority model.
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
    #[error("ProjectMemberAuthority: Database error.")]
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
            Self::ProjectMemberAuthorityId => t!("entities.project_member_authority.project_member_authority_id.name"),
            Self::Symbol => t!("entities.project_member_authority.symbol.name"),
            Self::Value => t!("entities.project_member_authority.value.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
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
impl ValidateExt for ActiveModel { type Error = Error; }


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
