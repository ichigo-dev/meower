//------------------------------------------------------------------------------
//! Option.
//------------------------------------------------------------------------------

mod props;

pub use props::OptgroupProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Optgroup<G: Html>( props: OptgroupProps<G> ) -> View<G>
{
    let classes = move ||
    {
        props.classes.get_clone() + " "
    };

    let children = props.children.call();
    view!
    {
        optgroup
        (
            class=classes(),
            name=props.name.get_clone(),
            value=props.value.get_clone(),
            disabled=props.disabled.get_clone(),
            ref=props.node_ref,
            ..props.attributes
        )
        {
            (children)
        }
    }
}
