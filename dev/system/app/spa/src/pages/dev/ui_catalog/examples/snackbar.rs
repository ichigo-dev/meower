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
    let open_snackbar_3 = create_signal(false);
    let open_snackbar_4 = create_signal(false);
    let open_snackbar_5 = create_signal(false);
    let open_snackbar_6 = create_signal(false);
    let open_snackbar_7 = create_signal(false);
    let open_snackbar_8 = create_signal(false);
    let open_snackbar_9 = create_signal(false);
    let open_snackbar_10 = create_signal(false);
    let open_snackbar_11 = create_signal(false);
    let open_snackbar_12 = create_signal(false);
    let open_snackbar_13 = create_signal(false);

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
                (t!("pages.dev.ui_catalog.snackbar.animations.heading"))
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
                    "Fade snackbar"
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_snackbar_3.set(true) },
                )
                {
                    "Open snackbar"
                }
                Snackbar
                (
                    animation=SnackbarAnimation::Slide.into(),
                    open=open_snackbar_3,
                    show_close_icon=BoolProp(true).into(),
                )
                {
                    "Slide snackbar"
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_snackbar_4.set(true) },
                )
                {
                    "Open snackbar"
                }
                Snackbar
                (
                    animation=SnackbarAnimation::Grow.into(),
                    open=open_snackbar_4,
                    show_close_icon=BoolProp(true).into(),
                )
                {
                    "Grow snackbar"
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_snackbar_5.set(true) },
                )
                {
                    "Open snackbar"
                }
                Snackbar
                (
                    animation=SnackbarAnimation::Flash.into(),
                    open=open_snackbar_5,
                    show_close_icon=BoolProp(true).into(),
                )
                {
                    "Flash snackbar"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.snackbar.colors.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color|
                    {
                        let open_snackbar = create_signal(false);
                        view!
                        {
                            button
                            (
                                class=&("ui_button ".to_string() + &color.get_class_name()),
                                on:click=move |_| { open_snackbar.set(true) },
                            )
                            {
                                "Open snackbar"
                            }
                            Snackbar
                            (
                                color=color.into(),
                                open=open_snackbar,
                                show_close_icon=BoolProp(true).into(),
                            )
                            {
                                "Snackbar"
                            }
                        }
                    }
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.snackbar.position.heading"))
            }
            div(class="flex flex_column flex_gap_md flex_align_center width_full")
            {
                div(class="flex flex_row flex_gap_md")
                {
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_6.set(true) },
                    )
                    {
                        "Top left"
                    }
                    Snackbar
                    (
                        open=open_snackbar_6,
                        position=SnackbarPosition::TopLeft.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_7.set(true) },
                    )
                    {
                        "Top"
                    }
                    Snackbar
                    (
                        open=open_snackbar_7,
                        position=SnackbarPosition::Top.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_8.set(true) },
                    )
                    {
                        "Top right"
                    }
                    Snackbar
                    (
                        open=open_snackbar_8,
                        position=SnackbarPosition::TopRight.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                }
                div(class="flex flex_row flex_gap_md")
                {
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_9.set(true) },
                    )
                    {
                        "Left"
                    }
                    Snackbar
                    (
                        open=open_snackbar_9,
                        position=SnackbarPosition::Left.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                    div(class="width_7xl")
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_10.set(true) },
                    )
                    {
                        "Right"
                    }
                    Snackbar
                    (
                        open=open_snackbar_10,
                        position=SnackbarPosition::Right.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                }
                div(class="flex flex_row flex_gap_md")
                {
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_11.set(true) },
                    )
                    {
                        "Bottom left"
                    }
                    Snackbar
                    (
                        open=open_snackbar_11,
                        position=SnackbarPosition::BottomLeft.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_12.set(true) },
                    )
                    {
                        "Bottom"
                    }
                    Snackbar
                    (
                        open=open_snackbar_12,
                        position=SnackbarPosition::Bottom.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                    button
                    (
                        class="ui_button primary width_7xl text_align_center",
                        on:click=move |_| { open_snackbar_13.set(true) },
                    )
                    {
                        "Bottom right"
                    }
                    Snackbar
                    (
                        open=open_snackbar_13,
                        position=SnackbarPosition::BottomRight.into(),
                        show_close_icon=BoolProp(true).into(),
                    )
                    {
                        "Snackbar"
                    }
                }
            }
        }
    }
}
