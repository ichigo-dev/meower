//------------------------------------------------------------------------------
//! Badge.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct BadgeProps<G: Html>
{
    #[prop(default)]
    pub badge_content: ReadSignal<usize>,

    pub children: View<G>,

    #[prop(default)]
    pub classes: String,

    #[prop(default)]
    pub color: String,

    #[prop(default)]
    pub invisible: bool,

    #[prop(default)]
    pub max: usize,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Badge<G: Html>( props: BadgeProps<G> ) -> View<G>
{
    let max = props.max.clone();
    let badge_content_view = if true
    {
            view!
            {
                span(class="badge_content") { (props.badge_content.get()) }
            }
    }
    else
    {
        view! { "" }
    };

    if props.invisible
    {
        return view! { (props.children) };
    }

    let classes = "ui_badge ".to_string() + &props.classes;
    view!
    {
        span(class=classes)
        {
            (badge_content_view)
            (props.children)
        }
    }
}
