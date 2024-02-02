//------------------------------------------------------------------------------
//! UI catalog page.
//------------------------------------------------------------------------------

mod examples;
use examples::*;

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
        Main(heading=t!("pages.dev.ui_catalog.heading_top"))
        {
            BadgeExamples(colors=*theme_colors)
            ChipExamples(colors=*theme_colors)
            HeadingExamples(colors=*theme_colors)
            IconExamples(colors=*theme_colors)
            ListExamples(colors=*theme_colors)
            TableExamples(colors=*theme_colors)
            TooltipExamples(colors=*theme_colors)
        }
    }
}
