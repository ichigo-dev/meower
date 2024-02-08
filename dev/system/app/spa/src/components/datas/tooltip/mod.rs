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
        let mut classes = vec!
        [
            "ui_tooltip".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.position.get().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        span(class=classes(), ..props.attributes)
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
