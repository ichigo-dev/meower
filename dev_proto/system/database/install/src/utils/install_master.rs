//------------------------------------------------------------------------------
//! Install master data utility.
//------------------------------------------------------------------------------

use meower_entity::workspace_member_authority::Map as WorkspaceMemberAuthorityMap;
use meower_entity::workspace_member_authority::ActiveModel as ActiveWorkspaceMemberAuthority;
use meower_entity::traits::validate::ValidateExt;

use std::error::Error;

use sea_orm::{ ActiveValue, ConnectionTrait };


//------------------------------------------------------------------------------
/// Installs master data for the application.
//------------------------------------------------------------------------------
pub async fn install_master<C>( hdb: &C ) -> Result<(), Box<dyn Error>>
where
    C: ConnectionTrait,
{
    // Creates workspace member authority.
    let authority_map = vec!
    [
        ("Admin", WorkspaceMemberAuthorityMap::Admin),
        ("General", WorkspaceMemberAuthorityMap::General),
        ("ReadOnly", WorkspaceMemberAuthorityMap::ReadOnly),
    ];
    for authority in authority_map
    {
        let symbol = authority.0.to_string();
        let value = authority.1 as i32;
        let workspace_member_authority = ActiveWorkspaceMemberAuthority
        {
            symbol: ActiveValue::Set(symbol),
            value: ActiveValue::Set(value),
            ..Default::default()
        };
        workspace_member_authority.validate_and_insert(hdb).await?;
    }
    Ok(())
}
