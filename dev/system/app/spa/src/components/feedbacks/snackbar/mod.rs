//------------------------------------------------------------------------------
//! Snackbar.
//------------------------------------------------------------------------------

mod animation;
mod position;
mod props;

pub use animation::SnackbarAnimation;
pub use position::SnackbarPosition;
pub use props::SnackbarProps;

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
        if props.open.get() && props.auto_hide_duration.get() > 0
        {
            let auto_hide_timer = Timeout::new
            (
                props.auto_hide_duration.get(),
                move || props.open.set(false),
            );
            auto_hide_timer.forget();
        }
    });

    let children = props.children.call();
    view!
    {
        span(class=classes(), ref=props.node_ref, ..props.attributes)
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
                            on:click=move |_| props.open.set(false),
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
