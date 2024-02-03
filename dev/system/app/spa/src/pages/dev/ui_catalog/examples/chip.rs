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
                Chip(label=StringProp("Chip").into())
                Chip
                (
                    label=StringProp("Chip").into(),
                    variant=ChipVariant::Outlined.into(),
                )
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
                Chip
                (
                    label=StringProp("Chip").into(),
                    disabled=BoolProp(true).into(),
                )
                Chip
                (
                    label=StringProp("Chip").into(),
                    variant=ChipVariant::Outlined.into(),
                    disabled=BoolProp(true).into(),
                )
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
                Chip
                (
                    label=StringProp("Chip").into(),
                    clickable=BoolProp(true).into(),
                )
                Chip
                (
                    label=StringProp("Chip").into(),
                    variant=ChipVariant::Outlined.into(),
                    clickable=BoolProp(true).into(),
                )
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
                Chip
                (
                    label=StringProp("Chip").into(),
                    left_icon=view! { span(class="ui_icon icon_envelope") }
                )
                Chip
                (
                    label=StringProp("Chip").into(),
                    variant=ChipVariant::Outlined.into(),
                    right_icon=view!
                    {
                        span(class="ui_icon icon_xmark clickable")
                    }
                )
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
                Chip
                (
                    label=StringProp("Chip").into(),
                    size=ChipSize::Small.into(),
                )
                Chip
                (
                    label=StringProp("Chip").into(),
                    size=ChipSize::Medium.into(),
                )
                Chip
                (
                    label=StringProp("Chip").into(),
                    size=ChipSize::Large.into(),
                )
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
                            Chip
                            (
                                label=StringProp("Chip").into(),
                                color=color.into(),
                            )
                            Chip
                            (
                                label=StringProp("Chip").into(),
                                color=color.into(),
                                variant=ChipVariant::Outlined.into(),
                            )
                        }
                    }
                }
            )
        }
    }
}
