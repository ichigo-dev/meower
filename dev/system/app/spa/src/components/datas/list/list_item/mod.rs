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
        let mut classes = vec!
        [
            "ui_list_item".to_string(),
            props.classes.get_clone(),
        ];
        if props.clickable.get() { classes.push("clickable".to_string()) }
        if props.divider.get() { classes.push("border_bottom".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
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
