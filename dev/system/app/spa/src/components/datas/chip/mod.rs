//------------------------------------------------------------------------------
//! Chip.
//------------------------------------------------------------------------------

mod props;
mod size;
mod variant;

pub use props::ChipProps;
pub use size::ChipSize;
pub use variant::ChipVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Chip<G: Html>( props: ChipProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_chip ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
            + &props.variant.get_clone().get_class_name() + " "
            + if props.clickable.get() { "clickable " } else { " " }
            + if props.disabled.get() { "disabled " } else { " " }
    };

    view!
    {
        span(class=classes(), ref=props.node_ref, ..props.attributes)
        {
            (props.left_icon)
            (props.label.get_clone())
            (props.right_icon)
        }
    }
}
