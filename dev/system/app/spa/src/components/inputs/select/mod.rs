//------------------------------------------------------------------------------
//! Select.
//------------------------------------------------------------------------------

mod optgroup;
mod option;
mod props;
mod size;
mod variant;

pub use optgroup::*;
pub use option::*;
pub use props::SelectProps;
pub use size::SelectSize;
pub use variant::SelectVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Select<G: Html>( props: SelectProps<G> ) -> View<G>
{
    let classes = move ||
    {
        "ui_select ".to_string()
            + &props.classes.get_clone() + " "
            + &props.color.get_clone().get_class_name() + " "
            + &props.size.get_clone().get_class_name() + " "
            + &props.variant.get_clone().get_class_name() + " "
    };

    let children = props.children.call();
    view!
    {
        div(class=classes())
        {
            select
            (
                name=props.name.get_clone(),
                value=props.value.get_clone(),
                disabled=props.disabled.get_clone(),
                ref=props.node_ref,
                ..props.attributes
            )
            {
                (children)
            }
        }
    }
}
