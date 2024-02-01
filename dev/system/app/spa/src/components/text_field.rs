//------------------------------------------------------------------------------
//! TextField component.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Clone, PartialEq, Prop)]
pub struct TextFieldProps
{
    pub id: String,
    pub label: String,
    pub placeholder: String,
    pub required: bool,
    pub textarea: bool,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub(crate) fn TextField<G: Html>( cx: Scope, props: TextFieldProps ) -> View<G>
{
    let id = create_signal(cx, props.id);
    if props.textarea
    {
        return view!
        {
            cx,
            label(for=id) { (props.label) }
            textarea
            (
                id=id,
                class="ui_text_field outlined",
                name=id,
                placeholder=props.placeholder,
                required=props.required,
            )
        }
    }
    else
    {
        return view!
        {
            cx,
            label(for=id) { (props.label) }
            input
            (
                id=id,
                class="ui_text_field outlined",
                name=id,
                placeholder=props.placeholder,
                required=props.required,
            )
        }
    }
}
