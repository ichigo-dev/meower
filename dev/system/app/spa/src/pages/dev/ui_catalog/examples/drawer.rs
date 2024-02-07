//------------------------------------------------------------------------------
//! DrawerExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn DrawerExamples<G: Html>() -> View<G>
{
    let open_drawer_1 = create_signal(false);
    let open_drawer_2 = create_signal(false);
    let open_drawer_3 = create_signal(false);
    let open_drawer_4 = create_signal(false);
    let open_drawer_5 = create_signal(false);

    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.drawer.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.drawer.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_drawer_1.set(true) },
                )
                {
                    "Open drawer"
                }
                Drawer(open=open_drawer_1)
                {
                    "Drawer content"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.drawer.position.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_justify_center flex_align_center width_full")
            {
                div(class="width_7xl")
                button
                (
                    class="ui_button primary width_7xl text_align_center",
                    on:click=move |_| { open_drawer_2.set(true) },
                )
                {
                    "Top"
                }
                Drawer
                (
                    open=open_drawer_2,
                    position=DrawerPosition::Top.into(),
                )
                {
                    "Drawer content"
                }
                div(class="width_7xl")
            }
            div(class="flex flex_row flex_gap_md flex_justify_center flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary width_7xl text_align_center",
                    on:click=move |_| { open_drawer_3.set(true) },
                )
                {
                    "Left"
                }
                Drawer
                (
                    open=open_drawer_3,
                    position=DrawerPosition::Left.into(),
                )
                {
                    "Drawer content"
                }
                div(class="width_7xl")
                button
                (
                    class="ui_button primary width_7xl text_align_center",
                    on:click=move |_| { open_drawer_4.set(true) },
                )
                {
                    "Right"
                }
                Drawer
                (
                    open=open_drawer_4,
                    position=DrawerPosition::Right.into(),
                )
                {
                    "Drawer content"
                }
            }
            div(class="flex flex_row flex_gap_md flex_justify_center flex_align_center width_full")
            {
                div(class="width_7xl")
                button
                (
                    class="ui_button primary width_7xl text_align_center",
                    on:click=move |_| { open_drawer_5.set(true) },
                )
                {
                    "Bottom"
                }
                Drawer
                (
                    open=open_drawer_5,
                    position=DrawerPosition::Bottom.into(),
                )
                {
                    "Drawer content"
                }
                div(class="width_7xl")
            }
        }
    }
}
