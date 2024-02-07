//------------------------------------------------------------------------------
//! Skeleton.
//------------------------------------------------------------------------------

mod animation;
mod props;
mod shape;

pub use animation::SkeletonAnimation;
pub use props::SkeletonProps;
pub use shape::SkeletonShape;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Skeleton<G: Html>( props: SkeletonProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_skeleton".to_string(),
            props.classes.get_clone(),
            props.animation.get_clone().get_class_name(),
            props.shape.get_clone().get_class_name(),
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
