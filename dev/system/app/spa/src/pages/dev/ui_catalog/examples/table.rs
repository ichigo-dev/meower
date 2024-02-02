//------------------------------------------------------------------------------
//! TableExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn TableExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.table.heading"))
        }
        MainPanel
        {
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Table()
                {
                    "hoge"
                }
            }
        }
    }
}
