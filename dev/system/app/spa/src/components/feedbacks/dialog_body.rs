//------------------------------------------------------------------------------
//! DialogBody.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct DialogBodyProps<G: Html>
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
pub fn DialogBody<G: Html>( props: DialogBodyProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "body ".to_string() + &props.classes.get_clone()
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
