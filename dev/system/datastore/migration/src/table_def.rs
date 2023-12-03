//------------------------------------------------------------------------------
//! Table definition.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;


//------------------------------------------------------------------------------
/// User.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum User
{
    Table,
    UserId,
    Email,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// UserAuth.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum UserAuth
{
    Table,
    UserAuthId,
    UserId,
    Password,
    CreatedAt,
    UpdatedAt,
}


//------------------------------------------------------------------------------
/// UserAccount.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum UserAccount
{
    Table,
    UserAccountId,
    UserId,
    UserAccountName,
    DisplayName,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// TemporaryUser.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum TemporaryUser
{
    Table,
    TemporaryUserId,
    Email,
    Password,
    CreatedAt,
}


//------------------------------------------------------------------------------
/// TemporaryUserToken.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum TemporaryUserToken
{
    Table,
    TemporaryUserTokenId,
    TemporaryUserId,
    Token,
    CreatedAt,
}


//------------------------------------------------------------------------------
/// Organization.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum Organization
{
    Table,
    OrganizationId,
    OrganizationName,
    DisplayName,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// OrganizationMember.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum OrganizationMember
{
    Table,
    OrganizationMemberId,
    OrganizationId,
    UserAccountId,
    Authority,
}
