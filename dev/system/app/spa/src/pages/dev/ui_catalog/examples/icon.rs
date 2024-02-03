//------------------------------------------------------------------------------
//! IconExamples.
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
pub fn IconExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.icon.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.icon.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon(icon=IconKind::Plus.into())
                Icon(icon=IconKind::Minus.into())
                Icon(icon=IconKind::CaretRight.into())
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.icon.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon
                (
                    icon=IconKind::Plus.into(),
                    size=IconSize::Small.into(),
                )
                Icon
                (
                    icon=IconKind::Minus.into(),
                    size=IconSize::Small.into(),
                )
                Icon
                (
                    icon=IconKind::CaretRight.into(),
                    size=IconSize::Small.into(),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon
                (
                    icon=IconKind::Plus.into(),
                    size=IconSize::Medium.into(),
                )
                Icon
                (
                    icon=IconKind::Minus.into(),
                    size=IconSize::Medium.into(),
                )
                Icon
                (
                    icon=IconKind::CaretRight.into(),
                    size=IconSize::Medium.into(),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon
                (
                    icon=IconKind::Plus.into(),
                    size=IconSize::Large.into(),
                )
                Icon
                (
                    icon=IconKind::Minus.into(),
                    size=IconSize::Large.into(),
                )
                Icon
                (
                    icon=IconKind::CaretRight.into(),
                    size=IconSize::Large.into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.icon.colors.heading"))
            }
            Indexed
            (
                iterable=colors,
                view=|color|
                {
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Icon
                            (
                                icon=IconKind::Plus.into(),
                                color=color.into(),
                            )
                            Icon
                            (
                                icon=IconKind::Minus.into(),
                                color=color.into(),
                            )
                            Icon
                            (
                                icon=IconKind::CaretRight.into(),
                                color=color.into(),
                            )
                        }
                    }
                }
            )
        }
    }
}
