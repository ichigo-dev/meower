//------------------------------------------------------------------------------
//! ButtonGroupItem.
//------------------------------------------------------------------------------

mod props;

pub use props::ButtonGroupItemProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn ButtonGroupItem<G: Html>( props: ButtonGroupItemProps<G> ) -> View<G>
{
    let classes = move ||
    {
        props.classes.get_clone() + " "
            + if props.active.get_clone() { "active " } else { " " }
    };

    let children = props.children.call();
    view!
    {
        button
        (
            class=classes(),
            ref=props.node_ref,
            disabled=props.disabled.get(),
            ..props.attributes
        )
        {
            (children)
        }
    }
}
