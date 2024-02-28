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
    Email,
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
    Token,
    AccountId,
    Name,
    Affiliation,
    Location,
    Email,
    Telno,
    Birthdate,
    Gender,
    Bio,
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
    FileKey,
    FileName,
    FileSize,
    ContentType,
    CreatedAt,
}


//------------------------------------------------------------------------------
/// AccountProfileCover.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum AccountProfileCover
{
    Table,
    AccountProfileCoverId,
    AccountProfileId,
    FileKey,
    FileName,
    FileSize,
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
    Name,
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
    Name,
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
