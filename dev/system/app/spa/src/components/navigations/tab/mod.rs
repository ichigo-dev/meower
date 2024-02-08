//------------------------------------------------------------------------------
//! Tab.
//------------------------------------------------------------------------------

mod props;
mod tab_item;
mod value;

pub use props::TabProps;
pub use tab_item::*;
pub use value::TabValue;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Tab<G: Html>( props: TabProps<G> ) -> View<G>
{
    provide_context(props.value);
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_tab".to_string(),
            props.classes.get_clone(),
            props.color.get_clone().get_class_name(),
        ];
        if props.vertical.get() { classes.push("vertical".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        div(class=classes(), ..props.attributes)
        {
            (children)
        }
    }
}
