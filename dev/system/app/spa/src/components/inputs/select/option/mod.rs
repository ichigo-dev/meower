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
        let mut classes = vec!
        [
            props.classes.get_clone(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        option
        (
            ref=props.node_ref,
            class=classes(),
            name=props.name.get_clone(),
            value=props.value.get_clone(),
            disabled=props.disabled.get(),
            selected=props.selected.get(),
            ..props.attributes
        )
        {
            (children)
        }
    }
}
