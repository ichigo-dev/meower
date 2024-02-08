//------------------------------------------------------------------------------
//! Drawer.
//------------------------------------------------------------------------------

mod props;
mod position;

pub use props::DrawerProps;
pub use position::DrawerPosition;

use crate::utils::props::*;

use sycamore::prelude::*;
use web_sys::MouseEvent;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Drawer<G: Html>( props: DrawerProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_drawer".to_string(),
            props.classes.get_clone(),
            props.position.get().get_class_name(),
        ];
        if props.open.get() { classes.push("open".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        div
        (
            class=classes(),
            on:click=move |_|
            {
                if props.close_on_backdrop.get()
                {
                    props.open.set(false)
                }
            }
        )
        {
            div
            (
                class="drawer",
                on:click=move |event: MouseEvent|
                {
                    event.stop_propagation();
                },
                ..props.attributes
            )
            {
                (children)
            }
        }
    }
}
