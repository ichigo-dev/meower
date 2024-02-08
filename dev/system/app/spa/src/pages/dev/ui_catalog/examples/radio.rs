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
pub fn RadioExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.radio.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.radio.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Radio(name=StrProp("radio1").into())
                Radio
                (
                    name=StrProp("radio1").into(),
                    checked=BoolProp(true).into(),
                )
                Radio(disabled=BoolProp(true).into())
                Radio
                (
                    checked=BoolProp(true).into(),
                    disabled=BoolProp(true).into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.radio.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Radio
                (
                    name=StrProp("radio2").into(),
                    checked=BoolProp(true).into(),
                    size=RadioSize::Small.into(),
                )
                Radio
                (
                    name=StrProp("radio2").into(),
                    size=RadioSize::Medium.into(),
                )
                Radio
                (
                    name=StrProp("radio2").into(),
                    size=RadioSize::Large.into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.radio.colors.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Radio
                        (
                            checked=BoolProp(true).into(),
                            color=color.into(),
                        )
                    }
                )
            }
        }
    }
}
