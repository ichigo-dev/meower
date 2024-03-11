//------------------------------------------------------------------------------
//! Group create page.
//------------------------------------------------------------------------------

use super::components::GroupForm;

use crate::AppState;
use crate::layouts::application::{ Layout, Main };

use rust_i18n::t;
use sycamore::prelude::*;
use sycamore_router::navigate;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Create<G: Html>() -> View<G>
{
    let state: AppState = use_context();
    if let None = state.selected_account.get_clone()
    {
        navigate("/");
        return view! {};
    };

    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.group.create.heading"))
            {
                GroupForm
            }
        }
    }
}
