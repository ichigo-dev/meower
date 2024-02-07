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
    let auto_hide_timer_id = create_signal(0i32);
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_snackbar".to_string(),
            props.classes.get_clone(),
            props.animation.get_clone().get_class_name(),
            props.position.get_clone().get_class_name(),
        ];
        if props.open.get() { classes.push("open".to_string()); }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    create_effect(move ||
    {
        if props.open.get()
        {
            if props.auto_hide_duration.get() > 0
            {
                let id = Timeout::new
                    (
                        props.auto_hide_duration.get(),
                        move ||
                        {
                            auto_hide_timer_id.set(0 as i32);
                            props.open.set(false);
                        }
                    )
                    .forget()
                    .as_f64()
                    .unwrap() as i32;
                auto_hide_timer_id.set(id);
            }
        }
    });

    let clear_timeout = move ||
    {
        let window = web_sys::window().unwrap();
        let id = auto_hide_timer_id.get();
        window.clear_timeout_with_handle(id);
    };
    on_cleanup(clear_timeout);

    let children = props.children.call();
    view!
    {
        span(class=classes(), ..props.attributes)
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
                                clear_timeout();
                                auto_hide_timer_id.set(0 as i32);
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
