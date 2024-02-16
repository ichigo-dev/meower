//------------------------------------------------------------------------------
//! Avatar.
//------------------------------------------------------------------------------

mod props;
mod size;

pub use props::AvatarProps;
pub use size::AvatarSize;

use crate::AppState;
use crate::utils::request::get;

use base64::prelude::*;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;


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

    spawn_local_scoped(async move
    {
        if let Some(file_key) = props.file_key.get_clone()
        {
            let file_key = if file_key.len() > 0
            {
                file_key
            }
            else
            {
                "default".to_string()
            };

            let mut state: AppState = use_context();
            let path = format!("account/avatar/{}", file_key);
            let avatar = get(&mut state, &path, "")
                .await
                .unwrap();
            let bytes = avatar.bytes().await.unwrap();
            let base64 = BASE64_STANDARD.encode(&bytes);
            props.base64.set(Some(base64));
        }
    });

    view!
    {
        img
        (
            class=classes(),
            src=src(),
            alt=props.alt,
            ..props.attributes
        )
    }
}
