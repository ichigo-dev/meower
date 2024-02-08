//------------------------------------------------------------------------------
//! ButtonGroupItem.
//------------------------------------------------------------------------------

mod props;

pub use props::ButtonGroupItemProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn ButtonGroupItem<G: Html>( props: ButtonGroupItemProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            props.classes.get_clone(),
            props.color.get().get_class_name(),
        ];
        if props.active.get() { classes.push("active".to_string()); }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        (
            match props.href.get_clone()
            {
                Some(href) if !props.disabled.get() =>
                {
                    let left_icon = props.left_icon.clone();
                    let children = children.clone();
                    let right_icon = props.right_icon.clone();
                    view!
                    {
                        a
                        (
                            class=classes(),
                            href=href,
                            rel="external",
                            ..props.attributes
                        )
                        {
                            div(class="flex flex_row flex_align_center flex_gap_md")
                            {
                                (left_icon)
                                (children)
                                (right_icon)
                            }
                        }
                    }
                },
                _ =>
                {
                    let left_icon = props.left_icon.clone();
                    let children = children.clone();
                    let right_icon = props.right_icon.clone();
                    view!
                    {
                        button
                        (
                            class=classes(),
                            disabled=props.disabled.get_clone(),
                            ..props.attributes
                        )
                        {
                            div(class="flex flex_row flex_align_center flex_gap_md")
                            {
                                (left_icon)
                                (children)
                                (right_icon)
                            }
                        }
                    }
                },
            }
        )
    }
}
