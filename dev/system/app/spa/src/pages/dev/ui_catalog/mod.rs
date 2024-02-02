//------------------------------------------------------------------------------
//! UI catalog page.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn UiCatalog<G: Html>() -> View<G>
{
    let mut content = create_signal(0usize);
    view!
    {
        button(on:click=move |_|{ content += 1 })
        { "UP" }
        Badge
        (
            badge_content=*content,
            children=view! { "hoge" }
        )
        Main
        (
            heading=t!("pages.dev.ui_catalog.heading"),
            children=view!
            {
                //--------------------------------------------------------------
                // Badge
                //--------------------------------------------------------------
                div(class="flex flex_column flex_gap_md width_full")
                {
                    h3(class="ui_heading divider")
                    {
                        (t!("pages.dev.ui_catalog.badge.heading"))
                    }
                    div(class="flex flex_row flex_gap_xl")
                    {
                        Badge
                        (
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                        Badge
                        (
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                        Badge
                        (
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                        Badge
                        (
                            invisible=true,
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                    }
                    div(class="flex flex_row flex_gap_xl")
                    {
                        Badge
                        (
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                        Badge
                        (
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                        Badge
                        (
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                        Badge
                        (
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                    }
                }
            }
        )
    }
}
