//------------------------------------------------------------------------------
//! Checkbox.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::CheckboxProps;
pub use size::CheckboxSize;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Checkbox<G: Html>( props: CheckboxProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_checkbox".to_string(),
            props.classes.get_clone(),
            props.color.get_clone().get_class_name(),
            props.size.get_clone().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    view!
    {
        input
        (
            class=classes(),
            type="checkbox",
            name=props.name.get_clone(),
            value=props.value.get_clone(),
            checked=props.checked.get(),
            disabled=props.disabled.get(),
            required=props.required.get(),
            ..props.attributes
        )
    }
}
