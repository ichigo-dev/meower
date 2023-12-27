//------------------------------------------------------------------------------
//! Mypage.
//------------------------------------------------------------------------------

use crate::apis::mypage;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let profile = mypage::get_profile(cx).await;

    view!
    {
        cx,
        h1 { "Mypage" }
        div
        {(
            match profile.get_data()
            {
                Ok(profile) =>
                {
                    view!
                    {
                        cx,
                        div { "Name: " (profile.name) }
                    }
                },
                Err(e) =>
                {
                    view! { cx, div { "Error: " (e) } }
                },
            }
        )}
        a(href="/", rel="external") { "Home" }
        a(href="/mypage/edit_profile", rel="external") { "Edit profile" }
    }
}
