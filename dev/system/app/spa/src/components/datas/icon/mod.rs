//------------------------------------------------------------------------------
//! Icon.
//------------------------------------------------------------------------------

mod kind;
mod props;
mod size;

pub use kind::IconKind;
pub use props::IconProps;
pub use size::IconSize;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Icon<G: Html>( props: IconProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_icon".to_string(),
            props.classes.get_clone(),
            props.icon.get().get_class_name(),
            props.color.get().get_class_name(),
            props.size.get().get_class_name(),
        ];
        if props.clickable.get() { classes.push("clickable".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    view!
    {
        span(ref=props.node_ref, class=classes(), ..props.attributes)
    }
}
