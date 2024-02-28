//------------------------------------------------------------------------------
//! Account detail page.
//------------------------------------------------------------------------------

mod account_profiles;
mod account_profile_card;

use account_profiles::AccountProfiles;

use crate::components::*;
use crate::layouts::application::{ Layout, Main };
use crate::utils::props::*;

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
        Layout
        {
            Main(heading=t!("pages.account.profile.heading"))
            {
                AccountProfiles(account_name=account_name)
                Button
                (
                    href=OptionProp(Some("/account/create_profile".to_string())).into(),
                    classes=StrProp("flex_align_self_end").into(),
                    round=ButtonRound::Full.into(),
                )
                {
                    (t!("pages.account.profile.button.create_profile"))
                }
            }
        }
    }
}
