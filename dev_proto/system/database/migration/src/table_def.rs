//------------------------------------------------------------------------------
//! Table definition.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;


//------------------------------------------------------------------------------
/// User.
//------------------------------------------------------------------------------
#[derive(Iden)]
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
#[derive(Iden)]
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
#[derive(Iden)]
pub(crate) enum UserAccount
{
    Table,
    UserAccountId,
    UserId,
    UserAccountName,
    DisplayName,
    CreatedAt,
    UpdatedAt,
    LastLoginedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// UserAccountProfile.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum UserAccountProfile
{
    Table,
    UserAccountProfileId,
    UserAccountId,
    Name,
    Biography,
    Company,
    Title,
    Location,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// TemporaryUser.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum TemporaryUser
{
    Table,
    TemporaryUserId,
    UserAccountName,
    Token,
    Email,
    Password,
    CreatedAt,
}


//------------------------------------------------------------------------------
/// TemporaryUserCode.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum TemporaryUserCode
{
    Table,
    TemporaryUserCodeId,
    TemporaryUserId,
    Code,
    CreatedAt,
    ExpiredAt,
}


//------------------------------------------------------------------------------
/// Organization.
//------------------------------------------------------------------------------
#[derive(Iden)]
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
#[derive(Iden)]
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
#[derive(Iden)]
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
#[derive(Iden)]
pub(crate) enum Workspace
{
    Table,
    WorkspaceId,
    WorkspaceName,
    DisplayName,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// WorkspaceMemberAuthority.
//------------------------------------------------------------------------------
#[derive(Iden)]
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
#[derive(Iden)]
pub(crate) enum WorkspaceMember
{
    Table,
    WorkspaceMemberId,
    WorkspaceId,
    UserAccountId,
    WorkspaceMemberAuthorityId,
}


//------------------------------------------------------------------------------
/// UserAccountWorkspace.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum UserAccountWorkspace
{
    Table,
    UserAccountWorkspaceId,
    UserAccountId,
    WorkspaceId,
}


//------------------------------------------------------------------------------
/// OrganizationWorkspace.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum OrganizationWorkspace
{
    Table,
    OrganizationWorkspaceId,
    OrganizationId,
    WorkspaceId,
}


//------------------------------------------------------------------------------
/// Project.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum Project
{
    Table,
    ProjectId,
    ProjectName,
    DisplayName,
    WorkspaceId,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// ProjectMemberAuthority.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum ProjectMemberAuthority
{
    Table,
    ProjectMemberAuthorityId,
    Symbol,
    Value,
}


//------------------------------------------------------------------------------
/// ProjectMember.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum ProjectMember
{
    Table,
    ProjectMemberId,
    ProjectId,
    UserAccountId,
    ProjectMemberAuthorityId,
}


//------------------------------------------------------------------------------
/// Task.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum Task
{
    Table,
    TaskId,
    ProjectId,
    OwnerUserAccountId,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}


//------------------------------------------------------------------------------
/// TaskMember.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum TaskMember
{
    Table,
    TaskMemberId,
    TaskId,
    UserAccountId,
}


//------------------------------------------------------------------------------
/// ResetPasswordToken.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum ResetPasswordToken
{
    Table,
    ResetPasswordTokenId,
    UserId,
    Token,
    CreatedAt,
    ExpiredAt,
}


//------------------------------------------------------------------------------
/// UserJwtSubject.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum UserJwtSubject
{
    Table,
    UserJwtSubjectId,
    UserId,
    Subject,
    CreatedAt,
}


//------------------------------------------------------------------------------
/// UserJwtTokenCode.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum UserJwtTokenCode
{
    Table,
    UserJwtTokenCodeId,
    UserId,
    Code,
    CreatedAt,
    ExpiredAt,
}
