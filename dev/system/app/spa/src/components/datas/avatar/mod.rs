//------------------------------------------------------------------------------
//! Avatar.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::AvatarProps;
pub use size::AvatarSize;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub async fn Avatar<G: Html>( props: AvatarProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_avatar".to_string(),
            props.classes.get_clone(),
            props.size.get().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let src = move ||
    {
        if let Some(base64) = props.base64.get_clone()
        {
            format!("data:{};base64,{}", props.mime_type, &base64)
        }
        else
        {
            props.src.get_clone()
        }
    };

    view!
    {
        img
        (
            ref=props.node_ref,
            class=classes(),
            src=src(),
            alt=props.alt,
            ..props.attributes
        )
    }
}
