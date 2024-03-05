//------------------------------------------------------------------------------
//! Create group form.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct GroupFormProps
{
    pub group_name: Option<String>,
    pub name: Option<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn GroupForm<G: Html>( props: GroupFormProps ) -> View<G>
{
    let save_handler = move |values: FormValues, _|
    {
    };

    view!
    {
        Form
        (
            classes=StrProp("flex flex_column flex_gap_md width_full").into(),
            on_submit=Box::new(save_handler),
        )
        {
            Label
            (
                label=t!("pages.account.group.components.group_form.group_name.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("name").into(),
                    placeholder=StringProp(t!("pages.account.group.components.group_form.group_name.placeholder")).into(),
                    required=BoolProp(true).into(),
                    value=StringProp(props.name.unwrap_or_default()).into(),
                )
            }
        }
    }
}
