//------------------------------------------------------------------------------
//! Heading.
//------------------------------------------------------------------------------

use sycamore::builder::prelude::*;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct HeadingProps<G: Html>
{
    #[prop(default)]
    align: ReadSignal<String>,

    children: View<G>,

    #[prop(default)]
    classes: ReadSignal<String>,

    #[prop(default)]
    color: ReadSignal<String>,

    #[prop(default)]
    level: ReadSignal<String>,

    #[prop(default)]
    thickness: ReadSignal<String>,

    #[prop(default)]
    variant: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Heading<G: Html>( props: HeadingProps<G> ) -> View<G>
{
    let classes = move ||
    {
        return "ui_heading ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.level.get_clone() + " "
            + &props.variant.get_clone() + " "
            + &props.thickness.get_clone() + " "
            + &props.align.get_clone();
    };

    view!
    {
        (
            match props.level.get_clone()
            {
                s if s == "h1".to_string() =>
                {
                    let children = props.children.clone();
                    view! { h1(class=classes()) { (children) } }
                },
                s if s == "h2".to_string() =>
                {
                    let children = props.children.clone();
                    view! { h2(class=classes()) { (children) } }
                },
                s if s == "h3".to_string() =>
                {
                    let children = props.children.clone();
                    view! { h3(class=classes()) { (children) } }
                },
                s if s == "h4".to_string() =>
                {
                    let children = props.children.clone();
                    view! { h4(class=classes()) { (children) } }
                },
                s if s == "h5".to_string() =>
                {
                    let children = props.children.clone();
                    view! { h5(class=classes()) { (children) } }
                },
                s if s == "h6".to_string() =>
                {
                    let children = props.children.clone();
                    view! { h6(class=classes()) { (children) } }
                },
                _ =>
                {
                    let children = props.children.clone();
                    view! { h3(class=classes()) { (children) } }
                },
            }
        )
    }
}
