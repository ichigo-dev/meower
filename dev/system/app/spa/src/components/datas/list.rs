//------------------------------------------------------------------------------
//! List.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct ListProps<G: Html>
{
    children: View<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<String>,

    #[prop(default)]
    pub ordered: ReadSignal<bool>,

    #[prop(default)]
    pub size: ReadSignal<String>,

    #[prop(default)]
    pub variant: ReadSignal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn List<G: Html>( props: ListProps<G> ) -> View<G>
{
    let classes = move ||
    {
        return "ui_list ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone() + " "
            + &props.size.get_clone() + " "
            + &props.variant.get_clone() + " "
            + if props.ordered.get() { "ordered " } else { "" };
    };

    view!
    {
        (
            if props.ordered.get()
            {
                let children = props.children.clone();
                view! { ol(class=classes()) { (children) } }
            }
            else
            {
                let children = props.children.clone();
                view! { ul(class=classes()) { (children) } }
            }
        )
    }
}
