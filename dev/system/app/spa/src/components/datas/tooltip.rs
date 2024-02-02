//------------------------------------------------------------------------------
//! Tooltip.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct TooltipProps<G: Html>
{
    children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    pub description: View<G>,

    #[prop(default)]
    pub position: ReadSignal<String>,

    #[prop(default = *create_signal(250))]
    pub max_width: ReadSignal<usize>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Tooltip<G: Html>( props: TooltipProps<G> ) -> View<G>
{
    let classes = move ||
    {
        return "ui_tooltip ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.position.get_clone();
    };

    let children = props.children.call();
    view!
    {
        span(class=classes())
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
