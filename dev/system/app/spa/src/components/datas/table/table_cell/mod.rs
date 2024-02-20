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
                        class=props.classes,
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
                        class=props.classes,
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
