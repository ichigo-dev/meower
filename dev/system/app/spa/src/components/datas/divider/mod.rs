//------------------------------------------------------------------------------
//! Divider.
//------------------------------------------------------------------------------

mod props;

pub use props::DividerProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Divider<G: Html>( props: DividerProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "width_full border_bottom margin_vertical_md".to_string(),
            props.classes.get_clone(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    view!
    {
        div(class=classes(), ref=props.node_ref, ..props.attributes)
    }
}
