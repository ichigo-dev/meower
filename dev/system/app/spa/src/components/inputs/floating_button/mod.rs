//------------------------------------------------------------------------------
//! FloatingButton.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::FloatingButtonProps;
pub use size::FloatingButtonSize;

use crate::components::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn FloatingButton<G: Html>( props: FloatingButtonProps<G> ) -> View<G>
{
    let form_values = try_use_context::<Signal<FormValues>>();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_floating_button".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.size.get().get_class_name(),
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
                    let icon = props.icon.clone();
                    let children = children.clone();
                    view!
                    {
                        a
                        (
                            ref=props.node_ref,
                            class=classes(),
                            href=href,
                            rel="external",
                            ..props.attributes
                        )
                        {
                            div(class="flex flex_row flex_align_center flex_gap_md")
                            {
                                (icon)
                                (children)
                            }
                        }
                    }
                },
                _ =>
                {
                    let icon = props.icon.clone();
                    let children = children.clone();
                    view!
                    {
                        button
                        (
                            ref=props.node_ref,
                            class=classes(),
                            disabled=props.disabled.get(),
                            name=props.name.get_clone(),
                            type=props.button_type.get_clone(),
                            on:click=move |_|
                            {
                                if let Some(form_values) = form_values
                                {
                                    let mut values = form_values.get_clone();
                                    if !props.disabled.get()
                                        && props.name.get_clone().len() > 0
                                        && props.button_type.get_clone() == "submit"
                                    {
                                        values.set
                                        (
                                            &props.name.get_clone(),
                                            &props.value.get_clone()
                                        );
                                    }
                                    else
                                    {
                                        values.remove(&props.name.get_clone());
                                    }
                                    form_values.set(values);
                                }
                            },
                            ..props.attributes
                        )
                        {
                            div(class="flex flex_row flex_align_center flex_gap_md")
                            {
                                (icon)
                                (children)
                            }
                        }
                    }
                }
            }
        )
    }
}
