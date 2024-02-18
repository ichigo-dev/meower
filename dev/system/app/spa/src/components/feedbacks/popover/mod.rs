//------------------------------------------------------------------------------
//! Popover.
//------------------------------------------------------------------------------

mod animation;
mod props;

pub use animation::PopoverAnimation;
pub use props::PopoverProps;

use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ Element, MouseEvent };


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Popover<G: Html>( props: PopoverProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_popover".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.animation.get().get_class_name(),
        ];
        if props.open.get() { classes.push("open".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let top = create_signal(0f64);
    let left = create_signal(0f64);
    create_effect(move ||
    {
        /*
        let dom_node = props.anchor.get::<DomNode>();
        let node = dom_node.to_web_sys();
        let elem = node.dyn_ref::<Element>().unwrap();
        let rect = elem.get_bounding_client_rect();
        top.set(rect.y());
        left.set(rect.x());
        */
    });

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
                class="popover",
                on:click=move |event: MouseEvent|
                {
                    event.stop_propagation();
                },
                style=format!("top: {}px; left: {}px;", top.get(), left.get()),
                ..props.attributes
            )
            {
                (children)
            }
        }
    }
}
