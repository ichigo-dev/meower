//------------------------------------------------------------------------------
//! CheckboxExamples.
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
pub fn CheckboxExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.checkbox.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.checkbox.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Checkbox
                Checkbox(checked=BoolProp(true).into())
                Checkbox(disabled=BoolProp(true).into())
                Checkbox
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
                (t!("pages.dev.ui_catalog.checkbox.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Checkbox
                (
                    checked=BoolProp(true).into(),
                    size=CheckboxSize::Small.into(),
                )
                Checkbox
                (
                    checked=BoolProp(true).into(),
                    size=CheckboxSize::Medium.into(),
                )
                Checkbox
                (
                    checked=BoolProp(true).into(),
                    size=CheckboxSize::Large.into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.checkbox.colors.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Checkbox
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
