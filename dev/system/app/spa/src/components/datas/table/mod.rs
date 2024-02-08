//------------------------------------------------------------------------------
//! Table.
//------------------------------------------------------------------------------

mod props;
mod size;
mod table_body;
mod table_cell;
mod table_foot;
mod table_head;
mod table_row;
mod variant;

pub use props::TableProps;
pub use size::TableSize;
pub use table_body::*;
pub use table_cell::*;
pub use table_foot::*;
pub use table_head::*;
pub use table_row::*;
pub use variant::TableVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Table<G: Html>( props: TableProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_table".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.size.get().get_class_name(),
            props.variant.get().get_class_name(),
        ];
        if props.sticky.get() { classes.push("sticky".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        table(class=classes(), ..props.attributes)
        {
            (children)
        }
    }
}
