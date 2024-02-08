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
        let mut classes = vec!
        [
            "ui_button_group".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.size.get().get_class_name(),
            props.variant.get().get_class_name(),
        ];
        if props.vertical.get() { classes.push("vertical".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        div(class=classes(), ..props.attributes)
        {
            (children)
        }
    }
}
