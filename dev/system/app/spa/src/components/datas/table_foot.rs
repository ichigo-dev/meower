//------------------------------------------------------------------------------
//! TableFoot.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct TableFootProps<G: Html>
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
pub fn TableFoot<G: Html>( props: TableFootProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        tfoot(class=props.classes)
        {
            (children)
        }
    }
}
