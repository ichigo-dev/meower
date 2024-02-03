//------------------------------------------------------------------------------
//! Dialog.
//------------------------------------------------------------------------------

mod animation;
mod dialog_body;
mod dialog_foot;
mod dialog_head;
mod props;
mod size;

pub use animation::DialogAnimation;
pub use dialog_body::*;
pub use dialog_foot::*;
pub use dialog_head::*;
pub use props::DialogProps;
pub use size::DialogSize;

use sycamore::prelude::*;
use web_sys::MouseEvent;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Dialog<G: Html>( props: DialogProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_dialog ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.animation.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
            + if props.open.get() { "open " } else { " " }
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
            dialog
            (
                class="dialog",
                on:click=move |event: MouseEvent|
                {
                    event.stop_propagation();
                }
            )
            {
                (children)
            }
        }
    }
}
