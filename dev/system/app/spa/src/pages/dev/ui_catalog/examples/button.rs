//------------------------------------------------------------------------------
//! ButtonExamples.
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
pub fn ButtonExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.button.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Button(variant=ButtonVariant::Text.into())
                {
                    "Button"
                }
                Button(variant=ButtonVariant::Filled.into())
                {
                    "Button"
                }
                Button(variant=ButtonVariant::Outlined.into())
                {
                    "Button"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button.disabled.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Button
                (
                    variant=ButtonVariant::Text.into(),
                    attr:disabled=true,
                )
                {
                    "Button"
                }
                Button
                (
                    variant=ButtonVariant::Filled.into(),
                    attr:disabled=true,
                )
                {
                    "Button"
                }
                Button
                (
                    variant=ButtonVariant::Outlined.into(),
                    attr:disabled=true,
                )
                {
                    "Button"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button.sizes.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                div(class="flex flex_row flex_gap_md flex_align_center width_full")
                {
                    Button
                    (
                        size=ButtonSize::Small.into(),
                        variant=ButtonVariant::Text.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        size=ButtonSize::Medium.into(),
                        variant=ButtonVariant::Text.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        size=ButtonSize::Large.into(),
                        variant=ButtonVariant::Text.into(),
                    )
                    {
                        "Button"
                    }
                }
                div(class="flex flex_row flex_gap_md flex_align_center width_full")
                {
                    Button
                    (
                        size=ButtonSize::Small.into(),
                        variant=ButtonVariant::Filled.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        size=ButtonSize::Medium.into(),
                        variant=ButtonVariant::Filled.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        size=ButtonSize::Large.into(),
                        variant=ButtonVariant::Filled.into(),
                    )
                    {
                        "Button"
                    }
                }
                div(class="flex flex_row flex_gap_md flex_align_center width_full")
                {
                    Button
                    (
                        size=ButtonSize::Small.into(),
                        variant=ButtonVariant::Outlined.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        size=ButtonSize::Medium.into(),
                        variant=ButtonVariant::Outlined.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        size=ButtonSize::Large.into(),
                        variant=ButtonVariant::Outlined.into(),
                    )
                    {
                        "Button"
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Button
                            (
                                color=color.into(),
                                variant=ButtonVariant::Text.into(),
                            )
                            {
                                "Button"
                            }
                            Button
                            (
                                color=color.into(),
                                variant=ButtonVariant::Filled.into(),
                            )
                            {
                                "Button"
                            }
                            Button
                            (
                                color=color.into(),
                                variant=ButtonVariant::Outlined.into(),
                            )
                            {
                                "Button"
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
                (t!("pages.dev.ui_catalog.button.rounds.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                div(class="flex flex_row flex_gap_md flex_align_center width_full")
                {
                    Button
                    (
                        variant=ButtonVariant::Filled.into(),
                        round=ButtonRound::Full.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        variant=ButtonVariant::Outlined.into(),
                        round=ButtonRound::Full.into(),
                    )
                    {
                        "Button"
                    }
                }
                div(class="flex flex_row flex_gap_md flex_align_center width_full")
                {
                    Button
                    (
                        variant=ButtonVariant::Filled.into(),
                        round=ButtonRound::None.into(),
                    )
                    {
                        "Button"
                    }
                    Button
                    (
                        variant=ButtonVariant::Outlined.into(),
                        round=ButtonRound::None.into(),
                    )
                    {
                        "Button"
                    }
                }
            }
        }
    }
}
