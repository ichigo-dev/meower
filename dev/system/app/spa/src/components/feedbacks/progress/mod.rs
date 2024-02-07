//------------------------------------------------------------------------------
//! Progress.
//------------------------------------------------------------------------------

mod props;
mod size;
mod thickness;
mod variant;

pub use props::ProgressProps;
pub use size::ProgressSize;
pub use thickness::ProgressThickness;
pub use variant::ProgressVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Progress<G: Html>( props: ProgressProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_progress".to_string(),
            props.classes.get_clone(),
            props.color.get_clone().get_class_name(),
            props.size.get_clone().get_class_name(),
            props.thickness.get_clone().get_class_name(),
            props.variant.get_clone().get_class_name(),
        ];
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
