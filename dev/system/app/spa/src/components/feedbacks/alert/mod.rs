//------------------------------------------------------------------------------
//! Alert.
//------------------------------------------------------------------------------

mod props;
mod severity;
mod variant;

pub use props::AlertProps;
pub use severity::AlertSeverity;
pub use variant::AlertVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Alert<G: Html>( props: AlertProps<G> ) -> View<G>
{
    let left_icon = props.left_icon.clone();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_alert".to_string(),
            props.classes.get_clone(),
            props.severity.get().get_class_name(),
            props.variant.get().get_class_name(),
        ];
        if left_icon.is_some() { classes.push("no_icon".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        div(class=classes(), ..props.attributes)
        {
            (props.left_icon)
            (children)
            span(class="flex_spacer")
            (props.right_icon)
        }
    }
}
