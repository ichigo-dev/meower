//------------------------------------------------------------------------------
//! Header.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::request::get;
use crate::utils::props::*;

use base64::prelude::*;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Header<G: Html>() -> View<G>
{
    let mut state: AppState = use_context();
    let avatar = get(&mut state, "account/avatar/default", "").await.unwrap();

    //let bytes = avatar.bytes().await.unwrap();
    //let base64 = BASE64_STANDARD.encode(bytes);

    view!
    {
        Box(classes=StrProp("ui_box primary color_primary_text text_align_center padding_sm fs_2xl").into())
        {
            "Meower"
            //img(src=("data:image/png;base64, ".to_string() + &base64))
        }
    }
}
