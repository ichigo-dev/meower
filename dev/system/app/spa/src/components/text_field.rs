//------------------------------------------------------------------------------
//! TextField component.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Clone, PartialEq, Props)]
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
pub fn TextField<G: Html>( props: TextFieldProps ) -> View<G>
{
    let id = create_signal(props.id);
    if props.textarea
    {
        return view!
        {
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
