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
pub fn UiCatalog<G: Html>() -> View<G>
{
    let theme_colors = create_signal(vec!
    [
        "primary".to_string(),
        "secondary".to_string(),
        "error".to_string(),
        "warning".to_string(),
        "info".to_string(),
        "success".to_string(),
    ]);

    view!
    {
        Main
        (
            heading=t!("pages.dev.ui_catalog.heading"),
            children=view!
            {
                BadgeExamples(colors=*theme_colors)
                ChipExamples(colors=*theme_colors)
            }
        )
    }
}


//------------------------------------------------------------------------------
/// BadgeExamples.
//------------------------------------------------------------------------------
#[component(inline_props)]
fn BadgeExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
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
                    badge_content=*create_signal("3".to_string()),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
                Badge
                (
                    badge_content=*create_signal("99999".to_string()),
                    max=*create_signal(999),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
                Badge
                (
                    invisible=*create_signal(true),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
                Badge
                (
                    badge_content=*create_signal("0".to_string()),
                    show_zero=*create_signal(true),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
            }
            div(class="flex flex_row flex_gap_xl")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Badge
                        (
                            badge_content=*create_signal("3".to_string()),
                            color=*create_signal(color),
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                    }
                )
            }
        }
    }
}


//------------------------------------------------------------------------------
/// ChipExamples.
//------------------------------------------------------------------------------
#[component(inline_props)]
fn ChipExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        div(class="flex flex_column flex_gap_md width_full")
        {
            h3(class="ui_heading divider")
            {
                (t!("pages.dev.ui_catalog.badge.heading"))
            }
            div(class="flex flex_row flex_gap_xl")
            {

            }
        }
    }
}
