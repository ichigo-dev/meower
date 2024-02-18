//------------------------------------------------------------------------------
//! PopoverExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;
use crate::variables::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn PopoverExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    let ref_1 = create_node_ref();
    let open_popover_1 = create_signal(false);

    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.popover.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.popover.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    ref=ref_1,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_1.set(true) },
                )
                {
                    "Open popover"
                }
                Popover(anchor=ref_1, open=open_popover_1)
                {
                    "Content"
                }
            }
        }
    }
}
