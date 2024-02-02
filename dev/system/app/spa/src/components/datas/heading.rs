//------------------------------------------------------------------------------
//! Heading.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct HeadingProps<G: Html>
{
    #[prop(default)]
    pub align: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub level: ReadSignal<String>,

    #[prop(default)]
    pub thickness: ReadSignal<String>,

    #[prop(default)]
    pub variant: ReadSignal<String>,
}


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
            + &props.color.get_clone() + " "
            + &props.level.get_clone() + " "
            + &props.variant.get_clone() + " "
            + &props.thickness.get_clone() + " "
            + &props.align.get_clone()
    };

    let children = props.children.call();
    view!
    {
        (
            match props.level.get_clone()
            {
                s if s == "h1".to_string() =>
                {
                    let children = children.clone();
                    view! { h1(class=classes()) { (children) } }
                },
                s if s == "h2".to_string() =>
                {
                    let children = children.clone();
                    view! { h2(class=classes()) { (children) } }
                },
                s if s == "h3".to_string() =>
                {
                    let children = children.clone();
                    view! { h3(class=classes()) { (children) } }
                },
                s if s == "h4".to_string() =>
                {
                    let children = children.clone();
                    view! { h4(class=classes()) { (children) } }
                },
                s if s == "h5".to_string() =>
                {
                    let children = children.clone();
                    view! { h5(class=classes()) { (children) } }
                },
                s if s == "h6".to_string() =>
                {
                    let children = children.clone();
                    view! { h6(class=classes()) { (children) } }
                },
                _ =>
                {
                    let children = children.clone();
                    view! { h3(class=classes()) { (children) } }
                },
            }
        )
    }
}
