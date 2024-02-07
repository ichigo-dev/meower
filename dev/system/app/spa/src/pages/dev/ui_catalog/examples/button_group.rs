//------------------------------------------------------------------------------
//! ButtonGroupExamples.
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
pub fn ButtonGroupExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.button_group.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button_group.basic.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                ButtonGroup(variant=ButtonGroupVariant::Text.into())
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup(variant=ButtonGroupVariant::Filled.into())
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup(variant=ButtonGroupVariant::Outlined.into())
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button_group.disabled.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                ButtonGroup(variant=ButtonGroupVariant::Text.into())
                {
                    ButtonGroupItem(attr:disabled=true)
                    {
                        "Button 1"
                    }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup(variant=ButtonGroupVariant::Filled.into())
                {
                    ButtonGroupItem(attr:disabled=true)
                    {
                        "Button 1"
                    }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup(variant=ButtonGroupVariant::Outlined.into())
                {
                    ButtonGroupItem(attr:disabled=true)
                    {
                        "Button 1"
                    }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button_group.sizes.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                ButtonGroup(size=ButtonGroupSize::Small.into())
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup(size=ButtonGroupSize::Medium.into())
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup(size=ButtonGroupSize::Large.into())
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button_group.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        ButtonGroup
                        (
                            color=color.into(),
                            variant=ButtonGroupVariant::Text.into(),
                        )
                        {
                            ButtonGroupItem { "Button 1" }
                            ButtonGroupItem { "Button 2" }
                            ButtonGroupItem { "Button 3" }
                        }
                        ButtonGroup
                        (
                            color=color.into(),
                            variant=ButtonGroupVariant::Filled.into(),
                        )
                        {
                            ButtonGroupItem { "Button 1" }
                            ButtonGroupItem { "Button 2" }
                            ButtonGroupItem { "Button 3" }
                        }
                        ButtonGroup
                        (
                            color=color.into(),
                            variant=ButtonGroupVariant::Outlined.into(),
                        )
                        {
                            ButtonGroupItem { "Button 1" }
                            ButtonGroupItem { "Button 2" }
                            ButtonGroupItem { "Button 3" }
                        }
                    },
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button_group.vertical.heading"))
            }
            div(class="flex flex_column flex_gap_md flex_align_start width_full")
            {
                ButtonGroup
                (
                    variant=ButtonGroupVariant::Text.into(),
                    vertical=BoolProp(true).into(),
                )
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup
                (
                    variant=ButtonGroupVariant::Filled.into(),
                    vertical=BoolProp(true).into(),
                )
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
                ButtonGroup
                (
                    variant=ButtonGroupVariant::Outlined.into(),
                    vertical=BoolProp(true).into(),
                )
                {
                    ButtonGroupItem { "Button 1" }
                    ButtonGroupItem { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.button_group.active.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                ButtonGroup(variant=ButtonGroupVariant::Text.into())
                {
                    ButtonGroupItem(active=BoolProp(true).into()) { "Button 1" }
                    ButtonGroupItem(active=BoolProp(true).into()) { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                    ButtonGroupItem { "Button 4" }
                }
                ButtonGroup(variant=ButtonGroupVariant::Filled.into())
                {
                    ButtonGroupItem(active=BoolProp(true).into()) { "Button 1" }
                    ButtonGroupItem(active=BoolProp(true).into()) { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                    ButtonGroupItem { "Button 4" }
                }
                ButtonGroup(variant=ButtonGroupVariant::Outlined.into())
                {
                    ButtonGroupItem(active=BoolProp(true).into()) { "Button 1" }
                    ButtonGroupItem(active=BoolProp(true).into()) { "Button 2" }
                    ButtonGroupItem { "Button 3" }
                    ButtonGroupItem { "Button 4" }
                }
            }
        }
    }
}
