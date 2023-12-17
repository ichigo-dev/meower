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
    view!
    {
        cx,
        h1 { "Mypage" }
        //div { (profile.name.clone().unwrap()) }
        a(href="/", rel="external") { "Home" }
        a(href="/mypage/edit_profile", rel="external") { "Edit profile" }
    }
}
