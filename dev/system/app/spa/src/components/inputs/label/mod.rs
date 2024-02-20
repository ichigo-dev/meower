//------------------------------------------------------------------------------
//! Label.
//------------------------------------------------------------------------------

mod direction;
mod props;

pub use direction::LabelDirection;
pub use props::LabelProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Label<G: Html>( props: LabelProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "width_full flex flex_gap_sm".to_string(),
            props.direction.get().get_class_name(),
            props.classes.get_clone(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };
    let children = props.children.call();
    view!
    {
        label(ref=props.node_ref, class=classes(), ..props.attributes)
        {
            span
            {
                (props.label)
                (
                    if props.required.get()
                    {
                        view! { span(class="color_error") { " *" } }
                    }
                    else
                    {
                        view! {}
                    }
                )
            }
            (children)
        }
    }
}
