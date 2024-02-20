//------------------------------------------------------------------------------
//! Heading.
//------------------------------------------------------------------------------

mod align;
mod level;
mod props;
mod thickness;
mod variant;

pub use align::HeadingAlign;
pub use level::HeadingLevel;
pub use props::HeadingProps;
pub use thickness::HeadingThickness;
pub use variant::HeadingVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Heading<G: Html>( props: HeadingProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_heading".to_string(),
            props.classes.get_clone(),
            props.align.get().get_class_name(),
            props.color.get().get_class_name(),
            props.level.get().get_class_name(),
            props.variant.get().get_class_name(),
            props.thickness.get().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        (
            match props.level.get_clone()
            {
                HeadingLevel::H1 =>
                {
                    let children = children.clone();
                    view!
                    {
                        h1
                        (
                            ref=props.node_ref,
                            class=classes(),
                            ..props.attributes
                        )
                        {
                            (children)
                        }
                    }
                },
                HeadingLevel::H2 =>
                {
                    let children = children.clone();
                    view!
                    {
                        h2
                        (
                            ref=props.node_ref,
                            class=classes(),
                            ..props.attributes
                        )
                        {
                            (children)
                        }
                    }
                },
                HeadingLevel::H3 =>
                {
                    let children = children.clone();
                    view!
                    {
                        h3
                        (
                            ref=props.node_ref,
                            class=classes(),
                            ..props.attributes
                        )
                        {
                            (children)
                        }
                    }
                },
                HeadingLevel::H4 =>
                {
                    let children = children.clone();
                    view!
                    {
                        h4
                        (
                            ref=props.node_ref,
                            class=classes(),
                            ..props.attributes
                        )
                        {
                            (children)
                        }
                    }
                },
                HeadingLevel::H5 =>
                {
                    let children = children.clone();
                    view!
                    {
                        h5
                        (
                            ref=props.node_ref,
                            class=classes(),
                            ..props.attributes
                        )
                        {
                            (children)
                        }
                    }
                },
                HeadingLevel::H6 =>
                {
                    let children = children.clone();
                    view!
                    {
                        h6
                        (
                            ref=props.node_ref,
                            class=classes(),
                            ..props.attributes
                        )
                        {
                            (children)
                        }
                    }
                },
            }
        )
    }
}
