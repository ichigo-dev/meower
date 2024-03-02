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
    let classes = move ||
    {
        let mut classes = vec!
        [
            "width_full height_7xl object_fit_cover".to_string(),
            props.classes.get_clone(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let src = move ||
    {
        if let Some(base64) = props.base64.get_clone()
        {
            base64
        }
        else
        {
            props.src.get_clone()
        }
    };

    create_effect(move ||
    {
        props.file_key.track();
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

                let bytes = cover.bytes().await.unwrap();
                let base64 = BASE64_STANDARD.encode(&bytes);
                let base64 = format!("data:{};base64,{}", content_type, base64);
                props.base64.set(Some(base64));
            }
        });
    });

    view!
    {
        img
        (
            ref=props.node_ref,
            class=classes(),
            src=src(),
            alt=props.alt,
            style="display: block;",
            ..props.attributes
        )
    }
}
