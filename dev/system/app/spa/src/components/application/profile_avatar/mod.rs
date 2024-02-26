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
            let headers = avatar.headers();
            let content_type = headers
                .get(header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap_or("image/png")
                .to_string();
            props.mime_type.set(content_type);

            let bytes = avatar.bytes().await.unwrap();
            let base64 = BASE64_STANDARD.encode(&bytes);
            props.base64.set(Some(base64));
        }
    });

    view!
    {
        Avatar
        (
            alt=props.alt,
            attributes=props.attributes,
            base64=props.base64,
            classes=props.classes,
            mime_type=*props.mime_type,
            node_ref=props.node_ref,
            size=props.size,
            src=props.src,
        )
    }
}
