//------------------------------------------------------------------------------
//! TableFoot.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
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
