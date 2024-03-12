//------------------------------------------------------------------------------
//! TableCell.
//------------------------------------------------------------------------------

mod props;

pub use props::TableCellProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TableCell<G: Html>( props: TableCellProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            props.classes.get_clone(),
        ];
        if props.min.get() { classes.push("min".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        (
            if props.is_head.get()
            {
                let children = children.clone();
                view!
                {
                    th
                    (
                        ref=props.node_ref,
                        class=classes(),
                        ..props.attributes
                    )
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
                    td
                    (
                        ref=props.node_ref,
                        class=classes(),
                        ..props.attributes
                    )
                    {
                        (children)
                    }
                }
            }
        )
    }
}
