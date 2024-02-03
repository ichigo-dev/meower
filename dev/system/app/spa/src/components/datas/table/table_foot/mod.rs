//------------------------------------------------------------------------------
//! TableFoot.
//------------------------------------------------------------------------------

mod props;

pub use props::TableFootProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TableFoot<G: Html>( props: TableFootProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        tfoot(class=props.classes)
        {
            (children)
        }
    }
}
