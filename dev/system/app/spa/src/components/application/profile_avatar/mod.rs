//------------------------------------------------------------------------------
//! Profile avatar.
//------------------------------------------------------------------------------

mod props;

pub use props::ProfileAvatarProps;

use crate::AppState;
use crate::components::*;
use crate::utils::request::get;

use base64::prelude::*;
use reqwest::header;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub async fn ProfileAvatar<G: Html>( props: ProfileAvatarProps<G> ) -> View<G>
{
    create_effect(move ||
    {
        props.file_key.track();
        spawn_local_scoped(async move
        {
            if props.base64.get_clone().is_some() || props.src.get_clone().len() > 0
            {
                return;
            }

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
                let headers = avatar.headers();
                let content_type = headers
                    .get(header::CONTENT_TYPE)
                    .unwrap()
                    .to_str()
                    .unwrap_or("image/png")
                    .to_string();

                let bytes = avatar.bytes().await.unwrap();
                let base64 = BASE64_STANDARD.encode(&bytes);
                let base64 = format!("data:{};base64,{}", content_type, base64);
                props.base64.set(Some(base64));
            }
        });
    });

    view!
    {
        Avatar
        (
            alt=props.alt,
            attributes=props.attributes,
            base64=props.base64,
            classes=props.classes,
            node_ref=props.node_ref,
            size=props.size,
            src=props.src,
        )
    }
}
