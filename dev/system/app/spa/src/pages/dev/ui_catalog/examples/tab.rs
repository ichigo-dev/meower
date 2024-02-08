//------------------------------------------------------------------------------
//! TabExamples.
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
pub fn TabExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.tab.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.tab.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Tab(value=TabValue("1").into())
                {
                    TabItem
                    (
                        active=BoolProp(true).into(),
                        value=TabValue("1").into(),
                    )
                    {
                        "Tab 1"
                    }
                    TabItem
                    (
                        value=TabValue("2").into(),
                    )
                    {
                        "Tab 2"
                    }
                    TabItem
                    (
                        disabled=BoolProp(true).into(),
                        value=TabValue("3").into(),
                    )
                    {
                        "Tab 3"
                    }
                    TabItem
                    (
                        value=TabValue("4").into(),
                    )
                    {
                        "Tab 4"
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.tab.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Tab
                        (
                            color=color.into(),
                            value=TabValue("1").into(),
                        )
                        {
                            TabItem
                            (
                                active=BoolProp(true).into(),
                                value=TabValue("1").into(),
                            )
                            {
                                "Tab 1"
                            }
                            TabItem
                            (
                                value=TabValue("2").into(),
                            )
                            {
                                "Tab 2"
                            }
                            TabItem
                            (
                                disabled=BoolProp(true).into(),
                                value=TabValue("3").into(),
                            )
                            {
                                "Tab 3"
                            }
                            TabItem
                            (
                                value=TabValue("4").into(),
                            )
                            {
                                "Tab 4"
                            }
                        }
                    }
                )
            }
        }
    }
}
