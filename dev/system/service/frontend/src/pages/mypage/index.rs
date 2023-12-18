//------------------------------------------------------------------------------
//! Mypage.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::api::mypage;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let app_state = use_context::<AppState>(cx);
    let profile = mypage::get_profile(&app_state).await;
    log::info!("profile: {:?}", profile);
    if profile.is_ng()
    {
        return view!
        {
            cx,
            h1 { "Mypage" }
            div { (profile.get_error()) }
        }
    }

    let profile = profile.get_data();
    let name = profile.name.clone();
    view!
    {
        cx,
        h1 { "Mypage" }
        div { (name) }
        a(href="/", rel="external") { "Home" }
        a(href="/mypage/edit_profile", rel="external") { "Edit profile" }
    }
}
