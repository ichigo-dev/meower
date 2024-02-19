//------------------------------------------------------------------------------
//! PopoverExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn PopoverExamples<G: Html>() -> View<G>
{
    let ref_1 = create_node_ref();
    let ref_2 = create_node_ref();
    let ref_3 = create_node_ref();
    let ref_4 = create_node_ref();
    let ref_5 = create_node_ref();
    let ref_6 = create_node_ref();
    let ref_7 = create_node_ref();
    let ref_8 = create_node_ref();
    let ref_9 = create_node_ref();
    let open_popover_1 = create_signal(false);
    let open_popover_2 = create_signal(false);
    let open_popover_3 = create_signal(false);
    let open_popover_4 = create_signal(false);
    let open_popover_5 = create_signal(false);
    let open_popover_6 = create_signal(false);
    let open_popover_7 = create_signal(false);
    let open_popover_8 = create_signal(false);
    let open_popover_9 = create_signal(false);

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
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.popover.position.heading"))
            }
            div(class="flex flex_column flex_gap_md flex_align_center width_full")
            {
                button
                (
                    ref=ref_2,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_2.set(true) },
                )
                {
                    "Top left"
                }
                Popover
                (
                    anchor=ref_2,
                    open=open_popover_2,
                    position=PopoverPosition::TopLeft.into(),
                )
                {
                    "Content"
                }
                button
                (
                    ref=ref_3,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_3.set(true) },
                )
                {
                    "Top"
                }
                Popover
                (
                    anchor=ref_3,
                    open=open_popover_3,
                    position=PopoverPosition::Top.into(),
                )
                {
                    "Content"
                }
                button
                (
                    ref=ref_4,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_4.set(true) },
                )
                {
                    "Top right"
                }
                Popover
                (
                    anchor=ref_4,
                    open=open_popover_4,
                    position=PopoverPosition::TopRight.into(),
                )
                {
                    "Content"
                }
                button
                (
                    ref=ref_5,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_5.set(true) },
                )
                {
                    "Left"
                }
                Popover
                (
                    anchor=ref_5,
                    open=open_popover_5,
                    position=PopoverPosition::Left.into(),
                )
                {
                    "Content"
                }
                button
                (
                    ref=ref_6,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_6.set(true) },
                )
                {
                    "Right"
                }
                Popover
                (
                    anchor=ref_6,
                    open=open_popover_6,
                    position=PopoverPosition::Right.into(),
                )
                {
                    "Content"
                }
                button
                (
                    ref=ref_7,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_7.set(true) },
                )
                {
                    "Bottom left"
                }
                Popover
                (
                    anchor=ref_7,
                    open=open_popover_7,
                    position=PopoverPosition::BottomLeft.into(),
                )
                {
                    "Content"
                }
                button
                (
                    ref=ref_8,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_8.set(true) },
                )
                {
                    "Bottom"
                }
                Popover
                (
                    anchor=ref_8,
                    open=open_popover_8,
                    position=PopoverPosition::Bottom.into(),
                )
                {
                    "Content"
                }
                button
                (
                    ref=ref_9,
                    class="ui_button primary",
                    on:click=move |_| { open_popover_9.set(true) },
                )
                {
                    "Bottom right"
                }
                Popover
                (
                    anchor=ref_9,
                    open=open_popover_9,
                    position=PopoverPosition::BottomRight.into(),
                )
                {
                    "Content"
                }
            }
        }
    }
}
