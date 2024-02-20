//------------------------------------------------------------------------------
//! TableRow.
//------------------------------------------------------------------------------

mod props;

pub use props::TableRowProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TableRow<G: Html>( props: TableRowProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        tr(ref=props.node_ref, class=props.classes, ..props.attributes)
        {
            (children)
        }
    }
}
