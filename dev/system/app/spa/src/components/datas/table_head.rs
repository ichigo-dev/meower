//------------------------------------------------------------------------------
//! TableHead.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct TableHeadProps<G: Html>
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
pub fn TableHead<G: Html>( props: TableHeadProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        thead(class=props.classes)
        {
            (children)
        }
    }
}
