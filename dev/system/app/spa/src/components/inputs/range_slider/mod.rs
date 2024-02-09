//------------------------------------------------------------------------------
//! RangeSlider.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::RangeSliderProps;
pub use size::RangeSliderSize;

use crate::components::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn RangeSlider<G: Html>( props: RangeSliderProps<G> ) -> View<G>
{
    let form_values = try_use_context::<Signal<FormValues>>();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_range_slider".to_string(),
            props.classes.get_clone(),
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
            if !props.disabled.get() && props.name.get_clone().len() > 0
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
            type="range",
            name=props.name.get_clone(),
            disabled=props.disabled.get(),
            required=props.required.get(),
            bind:value=props.value,
            ..props.attributes
        )
    }
}
