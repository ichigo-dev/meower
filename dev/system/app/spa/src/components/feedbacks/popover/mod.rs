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
    let node_ref = create_node_ref();
    let popover_node_ref = create_node_ref();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_popover".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.animation.get().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    create_effect(move ||
    {
        let dom_node = match node_ref.try_get::<DomNode>()
        {
            Some(dom_node) => dom_node,
            None => return,
        };
        let popover_dom_node = match popover_node_ref.try_get::<DomNode>()
        {
            Some(popover_dom_node) => popover_dom_node,
            None => return,
        };

        if props.open.get()
        {
            let anchor_dom_node = props.anchor.get::<DomNode>();
            let anchor_node = anchor_dom_node.to_web_sys();
            let anchor_elem = anchor_node.dyn_ref::<Element>().unwrap();
            let anchor_rect = anchor_elem.get_bounding_client_rect();
            let top = anchor_rect.top();
            let left = anchor_rect.left();

            popover_dom_node.set_property
            (
                "style",
                &format!("top: {}px; left: {}px;", top, left).into()
            );
            dom_node.add_class("open");
        }
        else
        {
            dom_node.remove_class("open");
        }
    });

    let children = props.children.call();
    view!
    {
        div
        (
            ref=node_ref,
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
                ref=popover_node_ref,
                class="popover",
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
