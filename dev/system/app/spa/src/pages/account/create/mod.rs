//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::Main;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Create<G: Html>() -> View<G>
{
    view!
    {
        Main(heading=t!("pages.account.create.heading"))
        {
            Form
            (
                classes=StrProp("flex flex_column flex_gap_md width_full").into(),
                on_submit=Box::new(|values: FormValues, _|
                {
                    log::info!("on_submit: {:?}", values);
                }),
            )
            {
                TextField(name=StrProp("account_name").into())
                Button(button_type=StrProp("submit").into())
                {
                    "Send"
                }
            }
        }
    }
}
