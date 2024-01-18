//------------------------------------------------------------------------------
//! Mypage.
//------------------------------------------------------------------------------

use crate::apis::mypage;

use rust_i18n::t;
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
        h1(class="ui_heading h1") { (t!("pages.mypage.index.heading")) }
        div
        {(
            match profile.get_data()
            {
                Ok(profile) =>
                {
                    view!
                    {
                        cx,
                        table(class="ui_table")
                        {
                            tbody
                            {
                                tr
                                {
                                    th
                                    {
                                        (t!("pages.mypage.index.profile_container.display_name"))
                                    }
                                    td
                                    {
                                        (profile.display_name)
                                    }
                                }
                            }
                        }
                    }
                },
                Err(e) =>
                {
                    view! { cx, div { "Error: " (e) } }
                },
            }
        )}
    }
}
