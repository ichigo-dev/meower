//------------------------------------------------------------------------------
//! ListItem.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct ListItemProps<G: Html>
{
    children: View<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub clickable: ReadSignal<bool>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn ListItem<G: Html>( props: ListItemProps<G> ) -> View<G>
{
    let classes = move ||
    {
        return "ui_list_item ".to_string()
            + &props.classes.get_clone() + " "
            + if props.clickable.get() { "clickable " } else { "" };
    };

    view!
    {
        li(class=classes())
        {
            (props.children)
        }
    }
}
