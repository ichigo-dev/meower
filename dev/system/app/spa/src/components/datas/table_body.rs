//------------------------------------------------------------------------------
//! TableBody.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
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
#[allow(dead_code)]
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
