//------------------------------------------------------------------------------
//! Alert.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct AlertProps<G: Html>
{
    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub severity: ReadSignal<String>,

    #[prop(default)]
    pub variant: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Alert<G: Html>( props: AlertProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_alert ".to_string()
            + &props.classes.get_clone() + " "
            + &props.severity.get_clone() + " "
            + &props.variant.get_clone()
    };

    let children = props.children.call();
    view!
    {
        div(class=classes())
        {
            (children)
        }
    }
}
