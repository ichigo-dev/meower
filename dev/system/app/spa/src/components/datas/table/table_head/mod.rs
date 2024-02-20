//------------------------------------------------------------------------------
//! TableHead.
//------------------------------------------------------------------------------

mod props;

pub use props::TableHeadProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TableHead<G: Html>( props: TableHeadProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        thead(ref=props.node_ref, class=props.classes, ..props.attributes)
        {
            (children)
        }
    }
}
