//------------------------------------------------------------------------------
//! Icon.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct IconProps
{
    pub icon: ReadSignal<String>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub size: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Icon<G: Html>( props: IconProps ) -> View<G>
{
    let classes = move ||
    {
        return "ui_icon ".to_string()
            + &props.classes.get_clone() + " "
            + "icon_" + &props.icon.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.size.get_clone();
    };

    view!
    {
        span(class=classes())
    }
}
