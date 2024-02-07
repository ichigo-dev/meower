//------------------------------------------------------------------------------
//! RadioExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn SelectExamples<G: Html>() -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.select.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.select.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Select(variant=SelectVariant::Default.into())
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select(variant=SelectVariant::Standard.into())
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select(variant=SelectVariant::Filled.into())
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.select.disabled.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Select
                (
                    variant=SelectVariant::Default.into(),
                    disabled=BoolProp(true).into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    variant=SelectVariant::Standard.into(),
                    disabled=BoolProp(true).into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    variant=SelectVariant::Filled.into(),
                    disabled=BoolProp(true).into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Select(variant=SelectVariant::Default.into())
                {
                    Option(disabled=BoolProp(true).into()) { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.select.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Select
                (
                    size=SelectSize::Small.into(),
                    variant=SelectVariant::Default.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    size=SelectSize::Medium.into(),
                    variant=SelectVariant::Default.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    size=SelectSize::Large.into(),
                    variant=SelectVariant::Default.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Select
                (
                    size=SelectSize::Small.into(),
                    variant=SelectVariant::Standard.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    size=SelectSize::Medium.into(),
                    variant=SelectVariant::Standard.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    size=SelectSize::Large.into(),
                    variant=SelectVariant::Standard.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Select
                (
                    size=SelectSize::Small.into(),
                    variant=SelectVariant::Filled.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    size=SelectSize::Medium.into(),
                    variant=SelectVariant::Filled.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
                Select
                (
                    size=SelectSize::Large.into(),
                    variant=SelectVariant::Filled.into(),
                )
                {
                    Option { "Option 1" }
                    Option { "Option 2" }
                    Option { "Option 3" }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.select.optgroup.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Select(variant=SelectVariant::Default.into())
                {
                    Optgroup(label=StringProp("Group 1").into())
                    {
                        Option { "Option 1" }
                        Option { "Option 2" }
                        Option { "Option 3" }
                    }
                    Optgroup(label=StringProp("Group 2").into())
                    {
                        Option { "Option 4" }
                        Option { "Option 5" }
                        Option { "Option 6" }
                    }
                    Optgroup
                    (
                        label=StringProp("Group 3").into(),
                        disabled=BoolProp(true).into(),
                    )
                    {
                        Option { "Option 7" }
                        Option { "Option 8" }
                        Option { "Option 9" }
                    }
                }
            }
        }
    }
}
