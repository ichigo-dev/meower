//------------------------------------------------------------------------------
//! List.
//------------------------------------------------------------------------------

mod list_item;
mod props;
mod variant;

pub use list_item::*;
pub use props::ListProps;
pub use variant::ListVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn List<G: Html>( props: ListProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_list".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.variant.get().get_class_name(),
        ];
        if props.ordered.get() { classes.push("ordered".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        (
            if props.ordered.get()
            {
                let children = children.clone();
                view!
                {
                    ol(ref=props.node_ref, class=classes(), ..props.attributes)
                    {
                        (children)
                    }
                }
            }
            else
            {
                let children = children.clone();
                view!
                {
                    ul(ref=props.node_ref, class=classes(), ..props.attributes)
                    {
                        (children)
                    }
                }
            }
        )
    }
}
