//------------------------------------------------------------------------------
//! TableCell.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct TableCellProps<G: Html>
{
    children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub is_head: ReadSignal<bool>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn TableCell<G: Html>( props: TableCellProps<G> ) -> View<G>
{
    let children = props.children.call();
    view!
    {
        (
            if props.is_head.get()
            {
                let children = children.clone();
                view! { th(class=props.classes) { (children) } }
            }
            else
            {
                let children = children.clone();
                view! { td(class=props.classes) { (children) } }
            }
        )
    }
}
