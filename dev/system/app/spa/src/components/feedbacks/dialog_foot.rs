//------------------------------------------------------------------------------
//! DialogFoot.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct DialogFootProps<G: Html>
{
    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn DialogFoot<G: Html>( props: DialogFootProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "foot ".to_string() + &props.classes.get_clone()
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
