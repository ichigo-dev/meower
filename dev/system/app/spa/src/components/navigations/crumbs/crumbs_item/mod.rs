//------------------------------------------------------------------------------
//! CrumbsItem.
//------------------------------------------------------------------------------

mod props;

pub use props::CrumbsItemProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn CrumbsItem<G: Html>( props: CrumbsItemProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            props.classes.get_clone(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        li(ref=props.node_ref, class=classes(), ..props.attributes)
        {
            (children)
        }
    }
}
