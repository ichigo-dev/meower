//------------------------------------------------------------------------------
//! RadioGroup.
//------------------------------------------------------------------------------

mod props;
mod radio;
mod value;

pub use props::RadioGroupProps;
pub use radio::*;
pub use value::RadioValue;

use crate::components::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn RadioGroup<G: Html>( props: RadioGroupProps<G> ) -> View<G>
{
    provide_context(props.value);
    let form_values = try_use_context::<Signal<FormValues>>();
    let classes = move ||
    {
        let mut classes = vec!
        [
            props.classes.get_clone(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    create_effect(move ||
    {
        if let Some(form_values) = form_values
        {
            let mut values = form_values.get_clone();
            if props.name.get_clone().len() > 0
            {
                values.set
                (
                    &props.name.get_clone(),
                    &props.value.get_clone().get(),
                );
            }
            else
            {
                values.remove(&props.name.get_clone());
            }
            form_values.set(values);
        }
    });

    let children = props.children.call();
    view!
    {
        div
        (
            class=classes(),
            name=props.name.get_clone(),
            ..props.attributes
        )
        {
            (children)
        }
    }
}
