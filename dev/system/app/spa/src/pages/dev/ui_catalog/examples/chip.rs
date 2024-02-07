//------------------------------------------------------------------------------
//! ChipExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;
use crate::variables::*;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn ChipExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.chip.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip { "Chip" }
                Chip(variant=ChipVariant::Outlined.into()) { "Chip" }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.disabled.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip(disabled=BoolProp(true).into()) { "Chip" }
                Chip
                (
                    variant=ChipVariant::Outlined.into(),
                    disabled=BoolProp(true).into(),
                )
                {
                    "Chip"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.clickable.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip(clickable=BoolProp(true).into()) { "Chip" }
                Chip
                (
                    variant=ChipVariant::Outlined.into(),
                    clickable=BoolProp(true).into(),
                )
                {
                    "Chip"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.icon.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip(left_icon=view! { span(class="ui_icon icon_envelope") })
                {
                    "Chip"
                }
                Chip
                (
                    variant=ChipVariant::Outlined.into(),
                    right_icon=view!
                    {
                        span(class="ui_icon icon_xmark clickable")
                    }
                )
                {
                    "Chip"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip(size=ChipSize::Small.into()) { "Chip" }
                Chip(size=ChipSize::Medium.into()) { "Chip" }
                Chip(size=ChipSize::Large.into()) { "Chip" }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.colors.heading"))
            }
            Indexed
            (
                iterable=colors,
                view=|color|
                {
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Chip(color=color.into()) { "Chip" }
                            Chip
                            (
                                color=color.into(),
                                variant=ChipVariant::Outlined.into(),
                            )
                            {
                                "Chip"
                            }
                        }
                    }
                }
            )
        }
    }
}
