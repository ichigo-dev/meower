//------------------------------------------------------------------------------
//! Button.
//------------------------------------------------------------------------------

mod props;
mod round;
mod size;
mod variant;

pub use props::ButtonProps;
pub use round::ButtonRound;
pub use size::ButtonSize;
pub use variant::ButtonVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Button<G: Html>( props: ButtonProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_button".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.round.get().get_class_name(),
            props.size.get().get_class_name(),
            props.variant.get().get_class_name(),
        ];
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
