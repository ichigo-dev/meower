//------------------------------------------------------------------------------
//! RangeSliderExamples.
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
pub fn RangeSliderExamples<G: Html>() -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.range_slider.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.range_slider.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                RangeSlider
                (
                    attr:min="0",
                    attr:max="100",
                )
                RangeSlider
                (
                    attr:min="0",
                    attr:max="100",
                    attr:step="10",
                )
                RangeSlider(disabled=BoolProp(true).into())
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.range_slider.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                RangeSlider
                (
                    attr:min="0",
                    attr:max="100",
                    size=RangeSliderSize::Small.into(),
                )
                RangeSlider
                (
                    attr:min="0",
                    attr:max="100",
                    size=RangeSliderSize::Medium.into(),
                )
                RangeSlider
                (
                    attr:min="0",
                    attr:max="100",
                    size=RangeSliderSize::Large.into(),
                )
            }
        }

    }
}
