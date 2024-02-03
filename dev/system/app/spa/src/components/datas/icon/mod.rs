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
        "ui_icon ".to_string()
            + &props.classes.get_clone() + " "
            + &props.icon.get_clone().get_class_name() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
            + if props.clickable.get_clone() { "clickable " } else { " " }
    };

    view!
    {
        span(class=classes(), ..props.attributes)
    }
}
