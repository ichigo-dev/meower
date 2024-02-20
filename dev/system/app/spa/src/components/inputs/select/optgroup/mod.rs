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
        optgroup
        (
            ref=props.node_ref,
            class=classes(),
            label=props.label.get_clone(),
            value=props.value.get_clone(),
            disabled=props.disabled.get(),
            ..props.attributes
        )
        {
            (children)
        }
    }
}
