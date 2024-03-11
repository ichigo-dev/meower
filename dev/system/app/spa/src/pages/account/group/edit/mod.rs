//------------------------------------------------------------------------------
//! Group edit page.
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
#[component(inline_props)]
pub fn Edit<G: Html>( group_name: String ) -> View<G>
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
            Main(heading=t!("pages.account.group.edit.heading"))
            {
                GroupForm
            }
        }
    }
}
