//------------------------------------------------------------------------------
//! Radio.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::RadioProps;
pub use size::RadioSize;

use crate::components::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Radio<G: Html>( props: RadioProps<G> ) -> View<G>
{
    let form_values = try_use_context::<Signal<FormValues>>();
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
        if let Some(form_values) = form_values
        {
            let mut values = form_values.get_clone();
            if !props.disabled.get()
            {
                values.set
                (
                    &props.name.get_clone(),
                    &props.value.get_clone()
                );
            }
            else
            {
                values.remove(&props.name.get_clone());
            }
            form_values.set(values);
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
