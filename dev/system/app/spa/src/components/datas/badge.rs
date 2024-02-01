//------------------------------------------------------------------------------
//! Badge.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Prop)]
pub struct BadgeProps<G: Html>
{
    #[builder(default)]
    pub badge_content: String,

    pub children: View<G>,

    #[builder(default)]
    pub classes: String,

    #[builder(default)]
    pub color: String,

    #[builder(default)]
    pub invisible: bool,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Badge<G: Html>( cx: Scope, props: BadgeProps<G> ) -> View<G>
{
    let badge_content = if props.badge_content.len() > 0
    {
        view! { cx, span(class="badge_content") { (props.badge_content) } }
    }
    else
    {
        view! { cx, "" }
    };

    view!
    {
        cx,
        span(class="ui_badge")
        {
            (badge_content)
            (props.children)
        }
    }
}
