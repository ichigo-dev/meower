//------------------------------------------------------------------------------
//! Tooltip.
//------------------------------------------------------------------------------

mod position;
mod props;

pub use position::TooltipPosition;
pub use props::TooltipProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Tooltip<G: Html>( props: TooltipProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_tooltip ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.position.get_clone().get_class_name() + " "
    };

    let children = props.children.call();
    view!
    {
        span(class=classes(), ref=props.node_ref, ..props.attributes)
        {
            span
            (
                class="description",
                style=format!("max-width: {}px;", props.max_width.get_clone()),
            )
            {
                span
                (
                    class="width_full",
                    style=
                    "
                    display: inline-block;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    ",
                )
                {
                    (props.description)
                }
            }
            (children)
        }
    }
}
