//------------------------------------------------------------------------------
//! ListItem.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ListItemProps<G: Html>
{
    children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub clickable: ReadSignal<bool>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn ListItem<G: Html>( props: ListItemProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_list_item ".to_string()
            + &props.classes.get_clone() + " "
            + if props.clickable.get() { "clickable " } else { "" }
    };

    let children = props.children.call();
    view!
    {
        li(class=classes())
        {
            (children)
        }
    }
}
