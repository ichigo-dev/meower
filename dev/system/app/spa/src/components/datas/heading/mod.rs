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
        "ui_heading ".to_string()
            + &props.classes.get_clone() + " "
            + &props.align.get_clone().get_class_name() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.level.get_clone().get_class_name() + " "
            + &props.variant.get_clone().get_class_name() + " "
            + &props.thickness.get_clone().get_class_name() + " "
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
                            class=classes(),
                            ref=props.node_ref,
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
                            class=classes(),
                            ref=props.node_ref,
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
                            class=classes(),
                            ref=props.node_ref,
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
                            class=classes(),
                            ref=props.node_ref,
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
                            class=classes(),
                            ref=props.node_ref,
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
                            class=classes(),
                            ref=props.node_ref,
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
