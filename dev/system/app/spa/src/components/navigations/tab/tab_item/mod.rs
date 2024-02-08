//------------------------------------------------------------------------------
//! TabItem.
//------------------------------------------------------------------------------

mod props;

pub use props::TabItemProps;

use crate::components::TabValue;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TabItem<G: Html>( props: TabItemProps<G> ) -> View<G>
{
    let value = try_use_context::<Signal<TabValue>>();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "tab".to_string(),
            props.classes.get_clone(),
        ];
        if props.active.get_clone() { classes.push("active".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    create_effect(move ||
    {
        if let Some(value) = value
        {
            props.active.set(props.value.get_clone() == value.get_clone());
        }
    });

    let children = props.children.call();
    view!
    {
        button
        (
            class=classes(),
            disabled=props.disabled.get(),
            on:click=move |_|
            {
                if let Some(value) = value
                {
                    value.set(props.value.get_clone());
                }
                props.active.set(true)
            },
            ..props.attributes
        )
        {
            (children)
        }
    }
}
