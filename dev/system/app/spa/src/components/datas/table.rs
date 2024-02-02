//------------------------------------------------------------------------------
//! Table.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct TableProps<G: Html>
{
    children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub size: ReadSignal<String>,

    #[prop(default)]
    pub sticky: ReadSignal<bool>,

    #[prop(default)]
    pub variant: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Table<G: Html>( props: TableProps<G> ) -> View<G>
{
    let classes = move ||
    {
        return "ui_table ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.size.get_clone() + " "
            + &props.variant.get_clone() + " "
            + if props.sticky.get() { "sticky " } else { "" };
    };

    let children = props.children.call();
    view!
    {
        table(class=classes())
        {
            (children)
        }
    }
}
