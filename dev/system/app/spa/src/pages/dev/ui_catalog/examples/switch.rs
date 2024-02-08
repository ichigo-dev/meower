//------------------------------------------------------------------------------
//! SwitchExamples.
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
pub fn SwitchExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.switch.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.switch.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Switch(name=StrProp("switch1").into())
                Switch
                (
                    name=StrProp("switch1").into(),
                    checked=BoolProp(true).into(),
                )
                Switch(disabled=BoolProp(true).into())
                Switch
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
                (t!("pages.dev.ui_catalog.switch.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Switch
                (
                    name=StrProp("switch2").into(),
                    checked=BoolProp(true).into(),
                    size=SwitchSize::Small.into(),
                )
                Switch
                (
                    name=StrProp("switch2").into(),
                    size=SwitchSize::Medium.into(),
                )
                Switch
                (
                    name=StrProp("switch2").into(),
                    size=SwitchSize::Large.into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.switch.colors.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Switch
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
