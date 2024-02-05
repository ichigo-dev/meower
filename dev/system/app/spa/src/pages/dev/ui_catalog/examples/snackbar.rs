//------------------------------------------------------------------------------
//! SnackbarExamples.
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
pub fn SnackbarExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    let open_snackbar_1 = create_signal(false);
    let open_snackbar_2 = create_signal(false);

    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.snackbar.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.snackbar.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_snackbar_1.set(true) },
                )
                {
                    "Open snackbar"
                }
                Snackbar
                (
                    open=open_snackbar_1,
                    show_close_icon=BoolProp(true).into(),
                )
                {
                    "Snackbar"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.snackbar.animation.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_snackbar_2.set(true) },
                )
                {
                    "Open snackbar"
                }
                Snackbar
                (
                    animation=SnackbarAnimation::Fade.into(),
                    open=open_snackbar_2,
                    show_close_icon=BoolProp(true).into(),
                )
                {
                    "Fade Snackbar"
                }
            }
        }
    }
}
