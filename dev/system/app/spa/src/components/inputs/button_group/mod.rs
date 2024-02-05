//------------------------------------------------------------------------------
//! ButtonGroup.
//------------------------------------------------------------------------------

mod button_group_item;
mod props;
mod size;
mod variant;

pub use button_group_item::*;
pub use props::ButtonGroupProps;
pub use size::ButtonGroupSize;
pub use variant::ButtonGroupVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn ButtonGroup<G: Html>( props: ButtonGroupProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_button_group ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
            + &props.variant.get_clone().get_class_name() + " "
            + if props.vertical.get_clone() { "vertical " } else { " " }
    };

    let children = props.children.call();
    view!
    {
        div(class=classes(), ref=props.node_ref, ..props.attributes)
        {
            (children)
        }
    }
}
