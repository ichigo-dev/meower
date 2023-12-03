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
/// OrganizationMemberAuthority.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum OrganizationMemberAuthority
{
    Table,
    OrganizationMemberAuthorityId,
    Symbol,
    Value,
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
    OrganizationMemberAuthorityId,
}


//------------------------------------------------------------------------------
/// Workspace.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum Workspace
{
    Table,
    WorkspaceId,
    WorkspaceName,
    DisplayName,
    OrganizationId,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// WorkspaceMemberAuthority.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum WorkspaceMemberAuthority
{
    Table,
    WorkspaceMemberAuthorityId,
    Symbol,
    Value,
}


//------------------------------------------------------------------------------
/// WorkspaceMember.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
pub(crate) enum WorkspaceMember
{
    Table,
    WorkspaceMemberId,
    WorkspaceId,
    UserAccountId,
    WorkspaceMemberAuthorityId,
}
