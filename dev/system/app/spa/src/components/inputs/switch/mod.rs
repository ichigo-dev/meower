//------------------------------------------------------------------------------
//! Switch.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::SwitchProps;
pub use size::SwitchSize;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Switch<G: Html>( props: SwitchProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_switch".to_string(),
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
            checked=props.checked.get_clone(),
            disabled=props.disabled.get_clone(),
            required=props.required.get_clone(),
            ..props.attributes
        )
    }
}
