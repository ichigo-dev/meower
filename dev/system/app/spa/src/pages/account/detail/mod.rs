//------------------------------------------------------------------------------
//! Account detail page.
//------------------------------------------------------------------------------

mod account_profile_tables;

use account_profile_tables::AccountProfileTables;

use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Detail<G: Html>( account_name: String ) -> View<G>
{
    view!
    {
        Main(heading=t!("pages.account.detail.heading"))
        {
            AccountProfileTables(account_name=account_name)
        }
    }
}
