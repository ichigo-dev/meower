//------------------------------------------------------------------------------
//! Table definition.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;


//------------------------------------------------------------------------------
/// Account.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum Account
{
    Table,
    AccountId,
    AccountName,
    PublicUserId,
    DefaultAccountProfileId,
    DefaultWorkspaceId,
    CreatedAt,
    LastLoginAt,
}


//------------------------------------------------------------------------------
/// AccountProfile.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum AccountProfile
{
    Table,
    AccountProfileId,
    AccountId,
    Name,
    Affiliation,
    Bio,
    Email,
    Birthdate,
    Gender,
    CreatedAt,
    UpdatedAt,
}


//------------------------------------------------------------------------------
/// AccountProfileAvatar.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum AccountProfileAvatar
{
    Table,
    AccountProfileAvatarId,
    AccountProfileId,
    FileSize,
    FileKey,
    ContentType,
    CreatedAt,
}


//------------------------------------------------------------------------------
/// Group.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum Group
{
    Table,
    GroupId,
    GroupName,
    CreatedAt,
    UpdatedAt,
}


//------------------------------------------------------------------------------
/// Group member.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum GroupMember
{
    Table,
    GroupMemberId,
    GroupId,
    AccountId,
    AccountProfileId,
    Role,
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
    CreatedAt,
    UpdatedAt,
}


//------------------------------------------------------------------------------
/// AccountWorkspace.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum AccountWorkspace
{
    Table,
    AccountWorkspaceId,
    WorkspaceId,
    AccountId,
}


//------------------------------------------------------------------------------
/// GroupWorkspace.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum GroupWorkspace
{
    Table,
    GroupWorkspaceId,
    WorkspaceId,
    GroupId,
}
