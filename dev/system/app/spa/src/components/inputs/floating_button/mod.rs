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
        let mut classes = vec!
        [
            "ui_floating_button".to_string(),
            props.classes.get_clone(),
            props.color.get_clone().get_class_name(),
            props.size.get_clone().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        button
        (
            class=classes(),
            disabled=props.disabled.get(),
            ..props.attributes
        )
        {
            (props.icon)
            (children)
        }
    }
}
