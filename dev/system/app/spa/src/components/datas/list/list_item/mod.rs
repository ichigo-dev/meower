//------------------------------------------------------------------------------
//! ListItem.
//------------------------------------------------------------------------------

mod props;

pub use props::ListItemProps;

use sycamore::prelude::*;


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
            + if props.clickable.get() { "clickable " } else { " " }
    };

    let children = props.children.call();
    view!
    {
        li(class=classes(), ..props.attributes)
        {
            (children)
        }
    }
}
