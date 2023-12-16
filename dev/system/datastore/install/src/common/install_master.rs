//------------------------------------------------------------------------------
/// Installs master data for the application.
//------------------------------------------------------------------------------

use meower_core::I18n;
use meower_entity::Validate;
use meower_entity::workspace_member_authority::AuthorityMap as WorkspaceMemberAuthorityMap;
use meower_entity::workspace_member_authority::ActiveModel as ActiveWorkspaceMemberAuthority;

use sea_orm::{ ActiveValue, ConnectionTrait };

pub async fn install_master<C>( hdb: &C, i18n: &I18n ) -> Result<(), String>
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
        if let Err(e) = workspace_member_authority
            .validate_and_insert(hdb, &i18n)
            .await
        {
            return Err(e);
        }
    }
    Ok(())
}
