//------------------------------------------------------------------------------
//! DialogBody.
//------------------------------------------------------------------------------

mod props;

pub use props::DialogBodyProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn DialogBody<G: Html>( props: DialogBodyProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "body".to_string(),
            props.classes.get_clone(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        div(class=classes(), ..props.attributes)
        {
            (children)
        }
    }
}
