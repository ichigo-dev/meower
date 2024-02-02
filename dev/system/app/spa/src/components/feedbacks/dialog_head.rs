//------------------------------------------------------------------------------
//! DialogHead.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct DialogHeadProps<G: Html>
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
pub fn DialogHead<G: Html>( props: DialogHeadProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "head ".to_string() + &props.classes.get_clone()
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
