//------------------------------------------------------------------------------
//! Crumbs.
//------------------------------------------------------------------------------

mod crumbs_item;
mod props;
mod variant;

pub use crumbs_item::*;
pub use props::CrumbsProps;
pub use variant::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Crumbs<G: Html>( props: CrumbsProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_crumbs".to_string(),
            props.classes.get_clone(),
            props.variant.get().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        ul(class=classes(), ..props.attributes)
        {
            (children)
        }
    }
}
