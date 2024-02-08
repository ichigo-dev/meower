//------------------------------------------------------------------------------
//! TabItem.
//------------------------------------------------------------------------------

mod props;

pub use props::TabItemProps;

use sycamore::prelude::*;
use web_sys::MouseEvent;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TabItem<G: Html>( props: TabItemProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "tab".to_string(),
            props.classes.get_clone(),
        ];
        if props.active.get_clone() { classes.push("active".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        button
        (
            class=classes(),
            disabled=props.disabled.get(),
            on:click=move |event: MouseEvent| { props.active.set(true) },
            ..props.attributes
        )
        {
            (children)
        }
    }
}
