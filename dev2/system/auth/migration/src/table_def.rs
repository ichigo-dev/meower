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
    JwtSubject,
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
/// TemporaryUser.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum TemporaryUser
{
    Table,
    TemporaryUserId,
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
/// JwtTokenCode.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum JwtTokenCode
{
    Table,
    JwtTokenCodeId,
    UserId,
    Code,
    CreatedAt,
    ExpiredAt,
}


//------------------------------------------------------------------------------
/// JwtRefreshToken.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum JwtRefreshToken
{
    Table,
    JwtRefreshTokenId,
    UserId,
    Token,
    CreatedAt,
    ExpiredAt,
}


//------------------------------------------------------------------------------
/// ClientApp.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum ClientApplication
{
    Table,
    ClientApplicationId,
    Name,
    ClientId,
    ClientSecret,
    RedirectUri,
    CreatedAt,
    UpdatedAt,
}
