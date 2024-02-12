//------------------------------------------------------------------------------
//! Radio.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::RadioProps;
pub use size::RadioSize;

use crate::components::RadioValue;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Radio<G: Html>( props: RadioProps<G> ) -> View<G>
{
    let radio_value = try_use_context::<Signal<RadioValue>>();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_radio".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.size.get().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    create_effect(move ||
    {
        if let Some(radio_value) = radio_value
        {
            if props.checked.get()
            {
                let value = props.value.get_clone();
                radio_value.set(RadioValue::new(&value));
            }
        }
    });

    view!
    {
        input
        (
            class=classes(),
            type="radio",
            name=props.name.get_clone(),
            disabled=props.disabled.get(),
            required=props.required.get(),
            bind:checked=props.checked,
            ..props.attributes
        )
    }
}
