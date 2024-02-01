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
    pub badge_content: String,

    pub children: View<G>,

    #[prop(default)]
    pub classes: String,

    #[prop(default)]
    pub color: String,

    #[prop(default)]
    pub invisible: bool,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Badge<G: Html>( props: BadgeProps<G> ) -> View<G>
{
    let badge_content = if props.badge_content.len() > 0
    {
        view! { span(class="badge_content") { (props.badge_content) } }
    }
    else
    {
        view! { "" }
    };

    view!
    {
        span(class="ui_badge")
        {
            (badge_content)
            (props.children)
        }
    }
}
