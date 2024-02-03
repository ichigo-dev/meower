//------------------------------------------------------------------------------
//! TableBody.
//------------------------------------------------------------------------------

mod props;

pub use props::TableBodyProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TableBody<G: Html>( props: TableBodyProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        tbody(class=props.classes)
        {
            (children)
        }
    }
}
