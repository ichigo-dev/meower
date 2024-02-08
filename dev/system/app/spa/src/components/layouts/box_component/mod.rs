//------------------------------------------------------------------------------
//! Box.
//------------------------------------------------------------------------------

mod props;
mod tag;

pub use props::BoxProps;
pub use tag::BoxTag;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Box<G: Html>( props: BoxProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_box".to_string(),
            props.classes.get_clone(),
            props.color.get_clone().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        (
            match props.tag.get_clone()
            {
                BoxTag::Div =>
                {
                    let children = children.clone();
                    view!
                    {
                        div(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Section =>
                {
                    let children = children.clone();
                    view!
                    {
                        section(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Article =>
                {
                    let children = children.clone();
                    view!
                    {
                        article(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Aside =>
                {
                    let children = children.clone();
                    view!
                    {
                        aside(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Header =>
                {
                    let children = children.clone();
                    view!
                    {
                        header(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Footer =>
                {
                    let children = children.clone();
                    view!
                    {
                        footer(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Main =>
                {
                    let children = children.clone();
                    view!
                    {
                        main(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Nav =>
                {
                    let children = children.clone();
                    view!
                    {
                        nav(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
                BoxTag::Span =>
                {
                    let children = children.clone();
                    view!
                    {
                        span(class=classes(), ..props.attributes)
                        {
                            (children)
                        }
                    }
                },
            }
        )
    }
}
