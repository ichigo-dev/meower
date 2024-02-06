//------------------------------------------------------------------------------
//! Radio.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::RadioProps;
pub use size::RadioSize;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Radio<G: Html>( props: RadioProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_radio ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
    };

    view!
    {
        input
        (
            class=classes(),
            type="radio",
            name=props.name.get_clone(),
            value=props.value.get_clone(),
            checked=props.checked.get(),
            disabled=props.disabled.get(),
            ref=props.node_ref,
            ..props.attributes
        )
    }
}
