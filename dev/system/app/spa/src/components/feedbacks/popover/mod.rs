//------------------------------------------------------------------------------
//! Popover.
//------------------------------------------------------------------------------

mod position;
mod props;

pub use position::PopoverPosition;
pub use props::PopoverProps;

use gloo_timers::callback::Timeout;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ Element, MouseEvent };

// Note that it needs to be changed according to the value of transition
// duration of `ui_popover`.
const ANIMATION_DURATION: u32 = 300;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Popover<G: Html>( props: PopoverProps<G> ) -> View<G>
{
    let node_ref = create_node_ref();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_popover".to_string(),
            props.classes.get_clone(),
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
        let popover_dom_node = match props.node_ref.try_get::<DomNode>()
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

            let popover_node = popover_dom_node.to_web_sys();
            let popover_elem = popover_node.dyn_ref::<Element>().unwrap();

            let (top, left) = match props.position.get()
            {
                PopoverPosition::TopLeft =>
                {
                    let top = anchor_rect.top()
                        - popover_elem.client_height() as f64;
                    let left = anchor_rect.left()
                        - popover_elem.client_width() as f64;
                    (top, left)
                },
                PopoverPosition::Top =>
                {
                    let top = anchor_rect.top()
                        - popover_elem.client_height() as f64;
                    let left = anchor_rect.left()
                        + anchor_elem.client_width() as f64 / 2.0
                        - popover_elem.client_width() as f64 / 2.0;
                    (top, left)
                },
                PopoverPosition::TopRight =>
                {
                    let top = anchor_rect.top()
                        - popover_elem.client_height() as f64;
                    let left = anchor_rect.left()
                        + anchor_elem.client_width() as f64;
                    (top, left)
                },
                PopoverPosition::Left =>
                {
                    let top = anchor_rect.top()
                        + anchor_elem.client_height() as f64 / 2.0
                        - popover_elem.client_height() as f64 / 2.0;
                    let left = anchor_rect.left()
                        - popover_elem.client_width() as f64;
                    (top, left)
                },
                PopoverPosition::Right =>
                {
                    let top = anchor_rect.top()
                        + anchor_elem.client_height() as f64 / 2.0
                        - popover_elem.client_height() as f64 / 2.0;
                    let left = anchor_rect.left()
                        + anchor_elem.client_width() as f64;
                    (top, left)
                },
                PopoverPosition::BottomLeft =>
                {
                    let top = anchor_rect.top()
                        + anchor_elem.client_height() as f64;
                    let left = anchor_rect.left()
                        - popover_elem.client_width() as f64;
                    (top, left)
                },
                PopoverPosition::Bottom =>
                {
                    let top = anchor_rect.top()
                        + anchor_elem.client_height() as f64;
                    let left = anchor_rect.left()
                        + anchor_elem.client_width() as f64 / 2.0
                        - popover_elem.client_width() as f64 / 2.0;
                    (top, left)
                },
                PopoverPosition::BottomRight =>
                {
                    let top = anchor_rect.top()
                        + anchor_elem.client_height() as f64;
                    let left = anchor_rect.left()
                        + anchor_elem.client_width() as f64;
                    (top, left)
                },
            };

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
            Timeout::new
            (
                ANIMATION_DURATION,
                move ||
                {
                    popover_dom_node.set_property
                    (
                        "style",
                        &"top: auto; left: auto;".into()
                    );
                }
            ).forget();
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
                ref=props.node_ref,
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
