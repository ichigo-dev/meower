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
        "foot ".to_string() + &props.classes.get_clone()
    };

    let children = props.children.call();
    view!
    {
        div(class=classes())
        {
            (children)
        }
    }
}
