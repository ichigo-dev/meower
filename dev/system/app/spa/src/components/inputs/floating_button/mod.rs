//------------------------------------------------------------------------------
//! FloatingButton.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::FloatingButtonProps;
pub use size::FloatingButtonSize;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn FloatingButton<G: Html>( props: FloatingButtonProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_floating_button ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
    };

    let children = props.children.call();
    view!
    {
        button
        (
            class=classes(),
            ref=props.node_ref,
            disabled=props.disabled.get(),
            ..props.attributes
        )
        {
            (props.icon)
            (children)
        }
    }
}
