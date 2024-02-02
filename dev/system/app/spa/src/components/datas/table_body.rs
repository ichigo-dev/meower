//------------------------------------------------------------------------------
//! TableBody.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct TableBodyProps<G: Html>
{
    children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn TableBody<G: Html>( props: TableBodyProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        tbody(class=props.classes)
        {
            (children)
        }
    }
}
