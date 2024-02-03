//------------------------------------------------------------------------------
//! DialogExamples.
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
pub fn DialogExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    let open_dialog_1 = create_signal(false);
    let open_dialog_2 = create_signal(false);
    let open_dialog_3 = create_signal(false);
    let open_dialog_4 = create_signal(false);
    let open_dialog_5 = create_signal(false);
    let open_dialog_6 = create_signal(false);
    let open_dialog_7 = create_signal(false);
    let open_dialog_8 = create_signal(false);
    let open_dialog_9 = create_signal(false);
    let open_dialog_10 = create_signal(false);

    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.dialog.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.dialog.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_1.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog(open=open_dialog_1)
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_1.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.dialog.animations.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_2.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog(open=open_dialog_2)
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_2.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_3.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog
                (
                    animation=DialogAnimation::Slide.into(),
                    open=open_dialog_3,
                )
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_3.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_4.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog
                (
                    animation=DialogAnimation::Grow.into(),
                    open=open_dialog_4,
                )
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_4.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_5.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog
                (
                    animation=DialogAnimation::Flash.into(),
                    open=open_dialog_5,
                )
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_5.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.dialog.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_6.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog
                (
                    size=DialogSize::Small.into(),
                    open=open_dialog_6,
                )
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_6.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_7.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog(open=open_dialog_7)
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_7.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_8.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog
                (
                    size=DialogSize::Large.into(),
                    open=open_dialog_8,
                )
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_8.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_9.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog
                (
                    size=DialogSize::Full.into(),
                    open=open_dialog_9,
                )
                {
                    DialogHead { "Dialog" }
                    DialogBody { "Content" }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_9.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.dialog.scroll.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                button
                (
                    class="ui_button primary",
                    on:click=move |_| { open_dialog_10.set(true) },
                )
                {
                    "Open dialog"
                }
                Dialog(open=open_dialog_10)
                {
                    DialogHead { "Dialog" }
                    DialogBody
                    {
                        div(class="flex flex_column flex_gap_md")
                        {
                            p { "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                            p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }
                            p { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                            p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                            p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                            p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                            p { "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                            p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt molli" }
                        }
                    }
                    DialogFoot
                    {
                        button
                        (
                            class="ui_button primary",
                            on:click=move |_| { open_dialog_10.set(false) },
                        )
                        {
                            "Close"
                        }
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.dialog.colors.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color|
                    {
                        let open_dialog = create_signal(false);
                        view!
                        {
                            button
                            (
                                class=&("ui_button ".to_string() + &color.get_class_name()),
                                on:click=move |_| { open_dialog.set(true) },
                            )
                            {
                                "Open dialog"
                            }
                            Dialog
                            (
                                color=color.into(),
                                open=open_dialog,
                            )
                            {
                                DialogHead { "Dialog" }
                                DialogBody { "Content" }
                                DialogFoot
                                {
                                    button
                                    (
                                        class="ui_button primary",
                                        on:click=move |_| { open_dialog.set(false) },
                                    )
                                    {
                                        "Close"
                                    }
                                }
                            }
                        }
                    }
                )
            }
        }
    }
}
