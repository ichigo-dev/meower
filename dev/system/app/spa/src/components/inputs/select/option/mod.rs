//------------------------------------------------------------------------------
//! Option.
//------------------------------------------------------------------------------

mod props;

pub use props::OptionProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Option<G: Html>( props: OptionProps<G> ) -> View<G>
{
    let classes = move ||
    {
        props.classes.get_clone() + " "
    };

    let children = props.children.call();
    view!
    {
        option
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
