//------------------------------------------------------------------------------
//! Group avatar.
//------------------------------------------------------------------------------

mod props;

pub use props::GroupAvatarProps;

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
pub async fn GroupAvatar<G: Html>( props: GroupAvatarProps<G> ) -> View<G>
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

                let state: AppState = use_context();
                let path = format!("account/group/avatar/{}", file_key);
                let avatar = get(&state, &path, "")
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

    let classes = create_signal("".to_string());
    create_effect(move ||
    {
        let c = props.classes.get_clone() + " radius_md";
        classes.set(c);
    });

    view!
    {
        Avatar
        (
            alt=props.alt,
            attributes=props.attributes,
            base64=props.base64,
            classes=*classes,
            node_ref=props.node_ref,
            size=props.size,
            src=props.src,
        )
    }
}
