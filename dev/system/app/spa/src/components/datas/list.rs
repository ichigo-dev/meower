//------------------------------------------------------------------------------
//! List.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ListProps<G: Html>
{
    children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub ordered: ReadSignal<bool>,

    #[prop(default)]
    pub variant: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn List<G: Html>( props: ListProps<G> ) -> View<G>
{
    let classes = move ||
    {
        return "ui_list ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.variant.get_clone() + " "
            + if props.ordered.get() { "ordered " } else { "" };
    };

    let children = props.children.call();
    view!
    {
        (
            if props.ordered.get()
            {
                let children = children.clone();
                view! { ol(class=classes()) { (children) } }
            }
            else
            {
                let children = children.clone();
                view! { ul(class=classes()) { (children) } }
            }
        )
    }
}
