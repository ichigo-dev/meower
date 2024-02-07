//------------------------------------------------------------------------------
//! RangeSlider.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::RangeSliderProps;
pub use size::RangeSliderSize;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn RangeSlider<G: Html>( props: RangeSliderProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_range_slider".to_string(),
            props.classes.get_clone(),
            props.size.get_clone().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    view!
    {
        input
        (
            class=classes(),
            type="range",
            name=props.name.get_clone(),
            value=props.value.get_clone(),
            disabled=props.disabled.get(),
            required=props.required.get(),
            ..props.attributes
        )
    }
}
