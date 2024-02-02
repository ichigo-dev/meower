//------------------------------------------------------------------------------
//! TableRow.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct TableRowProps<G: Html>
{
    children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn TableRow<G: Html>( props: TableRowProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        tr(class=props.classes)
        {
            (children)
        }
    }
}
