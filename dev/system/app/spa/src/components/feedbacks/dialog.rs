//------------------------------------------------------------------------------
//! Dialog.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct DialogProps<G: Html>
{
    #[prop(default)]
    pub animation: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default = *create_signal(true))]
    pub close_on_backdrop: ReadSignal<bool>,

    pub children: Children<G>,

    #[prop(default)]
    pub open: Signal<bool>,

    #[prop(default)]
    pub size: ReadSignal<String>,

    #[prop(default)]
    pub variant: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Dialog<G: Html>( props: DialogProps<G> ) -> View<G>
{
    let classes = move ||
    {
        log::info!("{}", props.open.get());
        "ui_dialog ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.animation.get_clone() + " "
            + &props.size.get_clone() + " "
            + &props.variant.get_clone() + " "
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
            dialog(class="dialog")
            {
                (children)
            }
        }
    }
}
