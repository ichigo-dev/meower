//------------------------------------------------------------------------------
//! CrumbsExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn CrumbsExamples<G: Html>() -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.crumbs.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.crumbs.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Crumbs
                {
                    CrumbsItem { "Item 1" }
                    CrumbsItem { "Item 2" }
                    CrumbsItem { "Item 3" }
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Crumbs(variant=CrumbsVariant::Angle.into())
                {
                    CrumbsItem { "Item 1" }
                    CrumbsItem { "Item 2" }
                    CrumbsItem { "Item 3" }
                }
            }
        }
    }
}
