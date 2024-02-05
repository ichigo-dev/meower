//------------------------------------------------------------------------------
//! Snackbar.
//------------------------------------------------------------------------------

mod animation;
mod position;
mod props;
mod provider;

pub use animation::SnackbarAnimation;
pub use position::SnackbarPosition;
pub use props::SnackbarProps;
pub use provider::{ SnackbarContext, SnackbarProvider };

use crate::components::{ Icon, IconKind };
use crate::utils::props::*;
use crate::variables::*;

use gloo_timers::callback::Timeout;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Snackbar<G: Html>( props: SnackbarProps<G> ) -> View<G>
{
    let context: SnackbarContext<G> = use_context();
    let node_ref = create_node_ref();
    let classes = move ||
    {
        "ui_snackbar ".to_string()
            + &props.classes.get_clone() + " "
            + &props.animation.get_clone().get_class_name() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.position.get_clone().get_class_name() + " "
            + if props.open.get_clone() { "open " } else { " " }
    };

    create_effect(move ||
    {
        if props.open.get()
        {
            if let Some(open_node_ref) = context.open_node_ref.get()
            {
                if open_node_ref != node_ref
                {
                    let id = context.auto_hide_timer_id.get();
                    let window = web_sys::window().unwrap();
                    window.clear_timeout_with_handle(id);
                    context.open_node_ref.set(None);
                    context.auto_hide_timer_id.set(0i32);
                    self.open.set(false);
                    return;
                }
            };
            context.open_node_ref.set(Some(node_ref));

            if props.auto_hide_duration.get() > 0
            {
                context.auto_hide_timer_id.set
                (
                    Timeout::new
                    (
                        props.auto_hide_duration.get(),
                        move ||
                        {
                            context.auto_hide_timer_id.set(0 as i32);
                            context.open_node_ref.set(None);
                            props.open.set(false);
                        }
                    )
                    .forget()
                    .as_f64()
                    .unwrap() as i32
                );
            }
        }
    });

    let children = props.children.call();
    view!
    {
        span
        (
            class=classes(),
            ref=node_ref,
            ref=props.node_ref,
            ..props.attributes
        )
        {
            (children)
            (
                if props.show_close_icon.get()
                {
                    view!
                    {
                        span(class="flex_spacer")
                        Icon
                        (
                            icon=IconKind::Xmark.into(),
                            color=Colors::Background.into(),
                            clickable=BoolProp(true).into(),
                            on:click=move |_|
                            {
                                let window = web_sys::window().unwrap();
                                window.clear_timeout_with_handle
                                (
                                    context.auto_hide_timer_id.get()
                                );
                                context.auto_hide_timer_id.set(0 as i32);
                                context.open_node_ref.set(None);
                                props.open.set(false);
                            },
                        )
                    }
                }
                else
                {
                    view!{}
                }
            )
        }
    }
}
