//------------------------------------------------------------------------------
//! BadgeExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn BadgeExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.badge.heading"))
        }
        MainPanel(children=view!
        {
            div(class="flex flex_row flex_gap_xl flex_align_center width_full")
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
            div(class="flex flex_row flex_gap_xl flex_align_center width_full")
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
        })
    }
}
