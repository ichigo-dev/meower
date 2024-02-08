//------------------------------------------------------------------------------
//! Box.
//------------------------------------------------------------------------------

mod props;

pub use props::LinkProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Link<G: Html>( props: LinkProps<G> ) -> View<G>
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
        a
        (
            href=props.href.get_clone(),
            rel="external",
            class=classes(),
            ..props.attributes
        )
        {
            (children)
        }
    }
}