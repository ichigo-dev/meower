//------------------------------------------------------------------------------
//! DialogHead.
//------------------------------------------------------------------------------

mod props;

pub use props::DialogHeadProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn DialogHead<G: Html>( props: DialogHeadProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "head".to_string(),
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
