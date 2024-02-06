//------------------------------------------------------------------------------
//! RadioExamples.
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
pub fn SelectExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
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
    }
}
