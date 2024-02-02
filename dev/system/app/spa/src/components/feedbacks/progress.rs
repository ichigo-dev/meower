//------------------------------------------------------------------------------
//! Progress.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ProgressProps<G: Html>
{
    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub size: ReadSignal<String>,
 
    #[prop(default)]
    pub thickness: ReadSignal<String>,

    #[prop(default = *create_signal("spin".to_string()))]
    pub variant: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Progress<G: Html>( props: ProgressProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_progress".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.size.get_clone() + " "
            + &props.thickness.get_clone() + " "
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
