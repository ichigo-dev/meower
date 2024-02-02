//------------------------------------------------------------------------------
//! Chip.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ChipProps<G: Html>
{
    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub clickable: ReadSignal<bool>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default)]
    pub label: ReadSignal<String>,

    #[prop(default)]
    pub left_icon: View<G>,

    #[prop(default)]
    pub right_icon: View<G>,

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
pub fn Chip<G: Html>( props: ChipProps<G> ) -> View<G>
{
    let classes = move ||
    {
        return "ui_chip ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.size.get_clone() + " "
            + &props.variant.get_clone() + " "
            + if props.clickable.get() { "clickable " } else { "" }
            + if props.disabled.get() { "disabled " } else { "" };
    };

    view!
    {
        span(class=classes())
        {
            (props.left_icon)
            (props.label.get_clone())
            (props.right_icon)
        }
    }
}
