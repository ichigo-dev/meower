//------------------------------------------------------------------------------
//! TextField.
//------------------------------------------------------------------------------

mod props;
mod size;
mod variant;

pub use props::TextFieldProps;
pub use size::TextFieldSize;
pub use variant::TextFieldVariant;

use crate::components::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn TextField<G: Html>( props: TextFieldProps<G> ) -> View<G>
{
    let form_values = try_use_context::<Signal<FormValues>>();
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_text_field".to_string(),
            props.classes.get_clone(),
            props.color.get().get_class_name(),
            props.size.get().get_class_name(),
            props.variant.get().get_class_name(),
        ];
        if props.full_width.get() { classes.push("full".to_string()) }
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
        (
            if props.multiline.get()
                || props.field_type.get_clone() == "textarea"
            {
                view!
                {
                    textarea
                    (
                        class=classes(),
                        name=props.name.get_clone(),
                        placeholder=props.placeholder.get_clone(),
                        disabled=props.disabled.get(),
                        readonly=props.readonly.get(),
                        required=props.required.get(),
                        rows=props.rows.get(),
                        bind:value=props.value,
                        ..props.attributes
                    )
                    {
                        (props.value.get_clone())
                    }
                }
            }
            else
            {
                view!
                {
                    input
                    (
                        class=classes(),
                        type=props.field_type.get_clone(),
                        name=props.name.get_clone(),
                        placeholder=props.placeholder.get_clone(),
                        disabled=props.disabled.get(),
                        readonly=props.readonly.get(),
                        required=props.required.get(),
                        bind:value=props.value,
                        ..props.attributes
                    )
                }
            }
        )
    }
}
