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
    IsPublic,
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
    Description,
    Representative,
    Location,
    Email,
    Telno,
    FoundedAt,
    CreatedAt,
    UpdatedAt,
    IsPublic,
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
}


//------------------------------------------------------------------------------
/// GroupAvatar.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum GroupAvatar
{
    Table,
    GroupAvatarId,
    GroupId,
    FileKey,
    FileName,
    FileSize,
    ContentType,
    CreatedAt,
}


//------------------------------------------------------------------------------
/// GroupCover.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum GroupCover
{
    Table,
    GroupCoverId,
    GroupId,
    FileKey,
    FileName,
    FileSize,
    ContentType,
    CreatedAt,
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


//------------------------------------------------------------------------------
/// CasbinRule.
///
/// This table is used as an adapter to store the access control policy of the
/// Casbin library.
///
/// reference: https://github.com/casbin-rs/sqlx-adapter
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum CasbinRule
{
    Table,
    Id,
    Ptype,
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
}
