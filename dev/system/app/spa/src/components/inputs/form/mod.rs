//------------------------------------------------------------------------------
//! Form.
//------------------------------------------------------------------------------

mod props;
mod values;

pub use props::FormProps;
pub use values::FormValues;

use sycamore::prelude::*;
use web_sys::SubmitEvent;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Form<G: Html>( props: FormProps<G> ) -> View<G>
{
    provide_context(props.values);
    let children = props.children.call();
    view!
    {
        form
        (
            ref=props.node_ref,
            action=props.action.get_clone(),
            class=props.classes.get_clone(),
            on:submit=move |event: SubmitEvent|
            {
                if let Some(on_submit) = props.on_submit.as_ref()
                {
                    on_submit(props.values.get_clone(), event.clone());
                }

                if !props.submit.get()
                {
                    event.prevent_default();
                }
            },
            ..props.attributes
        )
        {
            (children)
        }
    }
}
