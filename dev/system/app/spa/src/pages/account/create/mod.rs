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
    let form_values = create_signal(FormValues::new());
    create_effect(move ||
    {
        log::info!("effect: {:?}", form_values.get_clone());
    });
    view!
    {
        Main(heading=t!("pages.account.create.heading"))
        {
            Form
            (
                classes=StrProp("flex flex_column flex_gap_md width_full").into(),
                values=form_values,
            )
            {
                TextField(name=StrProp("text").into())
                Checkbox(name=StrProp("checkbox").into(), value=StrProp("checked").into())
                Radio(name=StrProp("radio").into(), value=StrProp("radio1").into(), checked=BoolProp(true).into())
                Radio(name=StrProp("radio").into(), value=StrProp("radio2").into())
                Radio(name=StrProp("radio").into(), value=StrProp("radio3").into())
                Select(name=StrProp("select").into())
                {
                    Option(value=StrProp("option1").into(), selected=BoolProp(true).into())
                    Option(value=StrProp("option2").into())
                    Option(value=StrProp("option3").into())
                }
                Switch(name=StrProp("switch").into(), value=StrProp("on").into())
                Button(name=StrProp("button").into(), value=StrProp("send").into(), button_type=StrProp("submit").into())
                {
                    "Send"
                }
            }
        }
    }
}
