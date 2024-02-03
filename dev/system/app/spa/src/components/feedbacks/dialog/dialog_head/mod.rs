//------------------------------------------------------------------------------
//! DialogHead.
//------------------------------------------------------------------------------

mod props;

pub use props::DialogHeadProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn DialogHead<G: Html>( props: DialogHeadProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "head ".to_string() + &props.classes.get_clone()
    };

    let children = props.children.call();
    view!
    {
        div(class=classes(), ref=props.node_ref, ..props.attributes)
        {
            (children)
        }
    }
}
