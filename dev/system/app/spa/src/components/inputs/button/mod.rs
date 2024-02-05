//------------------------------------------------------------------------------
//! Button.
//------------------------------------------------------------------------------

mod props;
mod round;
mod size;
mod variant;

pub use props::ButtonProps;
pub use round::ButtonRound;
pub use size::ButtonSize;
pub use variant::ButtonVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Button<G: Html>( props: ButtonProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_button ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.round.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
            + &props.variant.get_clone().get_class_name() + " "
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
            (children)
        }
    }
}
