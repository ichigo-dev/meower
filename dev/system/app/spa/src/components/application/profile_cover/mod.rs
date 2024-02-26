//------------------------------------------------------------------------------
//! Profile cover.
//------------------------------------------------------------------------------

mod props;

pub use props::ProfileCoverProps;

use crate::AppState;
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
pub async fn ProfileCover<G: Html>( props: ProfileCoverProps<G> ) -> View<G>
{
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
            let path = format!("account/cover/{}", file_key);
            let cover = get(&mut state, &path, "")
                .await
                .unwrap();
            let headers = cover.headers();
            let content_type = headers
                .get(header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap_or("image/png")
                .to_string();
            props.mime_type.set(content_type);

            let bytes = cover.bytes().await.unwrap();
            let base64 = BASE64_STANDARD.encode(&bytes);
            props.base64.set(Some(base64));
        }
    });

    view!
    {
        img
        (
            ref=props.node_ref,
            class=props.classes,
            src=src(),
            alt=props.alt,
            ..props.attributes
        )
    }
}
