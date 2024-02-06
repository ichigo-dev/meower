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
        "ui_checkbox ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
    };

    view!
    {
        input
        (
            type="checkbox",
            class=classes(),
            checked=props.checked.get(),
            disabled=props.disabled.get(),
            ref=props.node_ref,
            ..props.attributes
        )
    }
}
