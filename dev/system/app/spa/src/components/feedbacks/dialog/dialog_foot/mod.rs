//------------------------------------------------------------------------------
//! DialogFoot.
//------------------------------------------------------------------------------

mod props;

pub use props::DialogFootProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn DialogFoot<G: Html>( props: DialogFootProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "foot".to_string(),
            props.classes.get_clone(),
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
