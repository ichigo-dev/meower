//------------------------------------------------------------------------------
//! Badge.
//------------------------------------------------------------------------------

mod props;

pub use props::BadgeProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Badge<G: Html>( props: BadgeProps<G> ) -> View<G>
{
    let badge_content = move ||
    {
        if props.badge_content.get_clone() == "0" && !props.show_zero.get()
        {
            return "".to_string();
        }

        if props.max.get() > 0
        {
            if let Ok(num) = props.badge_content.get_clone().parse::<usize>()
            {
                if num > props.max.get()
                {
                    return props.max.get().to_string() + "+";
                }
                else
                {
                    return num.to_string();
                }
            }

            let content = props.badge_content.get_clone();
            let max_len = props.max.get().to_string().len();
            if content.len() > max_len
            {
                content.chars().take(max_len).collect::<String>() + "+"
            }
            else
            {
                content
            }
        }
        else
        {
            props.badge_content.get_clone()
        }
    };

    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_badge".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
        ];
        if props.invisible.get() { classes.push("hidden".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        span(class=classes(), ..props.attributes)
        {
            (
                if badge_content().len() > 0
                {
                    view! { span(class="badge_content") { (badge_content()) } }
                }
                else
                {
                    view! {}
                }
            )
            (children)
        }
    }
}
