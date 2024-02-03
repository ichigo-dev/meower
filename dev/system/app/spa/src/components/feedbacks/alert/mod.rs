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
    let classes = move ||
    {
        "ui_alert ".to_string()
            + &props.classes.get_clone() + " "
            + &props.severity.get_clone().get_class_name() + " "
            + &props.variant.get_clone().get_class_name() + " "
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
