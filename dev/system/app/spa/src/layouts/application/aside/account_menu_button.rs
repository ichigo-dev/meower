//------------------------------------------------------------------------------
//! Account menu button.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::props::*;
use crate::variables::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn AccountMenuButton<G: Html>
(
    node_ref: NodeRef<G>,
    open: Signal<bool>,
) -> View<G>
{
    let state: AppState = use_context();

    view!
    {
        (
            match state.selected_account.get_clone()
            {
                Some(selected_account) =>
                {
                    let file_key = selected_account.avatar_file_key.clone();
                    view!
                    {
                        MiniProfile
                        (
                            clickable=BoolProp(true).into(),
                            name=selected_account.name.clone(),
                            account_name=selected_account.name.clone(),
                            file_key=file_key,
                            node_ref=node_ref,
                            on:click=move |_|
                            {
                                open.set(!open.get());
                            },
                        )
                    }
                },
                None =>
                {
                    view!
                    {
                        Button
                        (
                            color=Colors::Primary.into(),
                            size=ButtonSize::Small.into(),
                            href=OptionProp(Some("/account/create".to_string())).into(),
                        )
                        {
                            (t!("common.aside.account_menu_button.button.create"))
                        }
                    }
                },
            }
        )
    }
}
