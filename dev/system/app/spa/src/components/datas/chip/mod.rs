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
        let mut classes = vec!
        [
            "ui_chip".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.size.get().get_class_name(),
            props.variant.get().get_class_name(),
        ];
        if props.clickable.get() { classes.push("clickable".to_string()) }
        if props.disabled.get() { classes.push("disabled".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        span(ref=props.node_ref, class=classes(), ..props.attributes)
        {
            (props.left_icon)
            (children)
            (props.right_icon)
        }
    }
}
