//------------------------------------------------------------------------------
//! BadgeExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;
use crate::utils::props::*;
use crate::variables::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn BadgeExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.badge.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.badge.basic.heading"))
            }
            div(class="flex flex_row flex_gap_xl flex_align_center width_full")
            {
                Badge
                {
                    div(class="ui_icon icon_envelope")
                }
                Badge(badge_content=StringProp("3").into())
                {
                    div(class="ui_icon icon_envelope")
                }
                Badge
                (
                    badge_content=StringProp("99999").into(),
                    max=UsizeProp(999).into(),
                )
                {
                    div(class="ui_icon icon_envelope")
                }
                Badge(invisible=BoolProp(true).into())
                {
                    div(class="ui_icon icon_envelope")
                }
                Badge
                (
                    badge_content=StringProp("0").into(),
                    show_zero=BoolProp(true).into(),
                )
                {
                    div(class="ui_icon icon_envelope")
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.badge.colors.heading"))
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
                            badge_content=StringProp("3").into(),
                            color=color.into(),
                        )
                        {
                            div(class="ui_icon icon_envelope")
                        }
                    }
                )
            }
        }
    }
}
