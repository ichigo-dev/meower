//------------------------------------------------------------------------------
//! Table definition.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;


//------------------------------------------------------------------------------
/// UserToken.
//------------------------------------------------------------------------------
#[derive(Iden)]
pub(crate) enum UserToken
{
    Table,
    UserTokenId,
    Token,
    PublicUserId,
    UserEmail,
    AccessToken,
    RefreshToken,
    CreatedAt,
    ExpiredAt,
}
