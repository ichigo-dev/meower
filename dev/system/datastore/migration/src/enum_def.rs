//------------------------------------------------------------------------------
//! Enum definition.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::EnumIter;


//------------------------------------------------------------------------------
/// OrganizationMemberAuthority.
//------------------------------------------------------------------------------
#[derive(Iden, EnumIter)]
pub enum OrganizationMemberAuthority
{
    Table,
    #[iden = "1"]
    Member,
    #[iden = "99"]
    Admin,
}
