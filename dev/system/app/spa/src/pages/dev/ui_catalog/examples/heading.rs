//------------------------------------------------------------------------------
//! HeadingExamples.
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
pub fn HeadingExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.heading.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Heading(level=HeadingLevel::H1.into()) { "Heading" }
                Heading(level=HeadingLevel::H2.into()) { "Heading" }
                Heading(level=HeadingLevel::H3.into()) { "Heading" }
                Heading(level=HeadingLevel::H4.into()) { "Heading" }
                Heading(level=HeadingLevel::H5.into()) { "Heading" }
                Heading(level=HeadingLevel::H6.into()) { "Heading" }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.divider.align.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=HeadingVariant::Divider.into(),
                    align=HeadingAlign::Left.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Divider.into(),
                    align=HeadingAlign::Center.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Divider.into(),
                    align=HeadingAlign::Right.into(),
                )
                {
                    "Heading"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.divider.thickness.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=HeadingVariant::Divider.into(),
                    thickness=HeadingThickness::Thin.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Divider.into(),
                    thickness=HeadingThickness::Normal.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Divider.into(),
                    thickness=HeadingThickness::Thick.into(),
                )
                {
                    "Heading"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.divider.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=HeadingVariant::Divider.into(),
                            color=color.into(),
                        )
                        {
                            "Heading"
                        }
                    }
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.bullet.thickness.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=HeadingVariant::Bullet.into(),
                    thickness=HeadingThickness::Thin.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Bullet.into(),
                    thickness=HeadingThickness::Normal.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Bullet.into(),
                    thickness=HeadingThickness::Thick.into(),
                )
                {
                    "Heading"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.bullet.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=HeadingVariant::Bullet.into(),
                            color=color.into(),
                        )
                        {
                            "Heading"
                        }
                    }
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.line.align.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=HeadingVariant::Line.into(),
                    align=HeadingAlign::Left.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Line.into(),
                    align=HeadingAlign::Center.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Line.into(),
                    align=HeadingAlign::Right.into(),
                )
                {
                    "Heading"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.line.thickness.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=HeadingVariant::Line.into(),
                    thickness=HeadingThickness::Thin.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Line.into(),
                    thickness=HeadingThickness::Normal.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Line.into(),
                    thickness=HeadingThickness::Thick.into(),
                )
                {
                    "Heading"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.line.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=HeadingVariant::Line.into(),
                            color=color.into(),
                        )
                        {
                            "Heading"
                        }
                    }
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.band.align.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=HeadingVariant::Band.into(),
                    align=HeadingAlign::Left.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Band.into(),
                    align=HeadingAlign::Center.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Band.into(),
                    align=HeadingAlign::Right.into(),
                )
                {
                    "Heading"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.band.thickness.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=HeadingVariant::Band.into(),
                    thickness=HeadingThickness::Thin.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Band.into(),
                    thickness=HeadingThickness::Normal.into(),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=HeadingVariant::Band.into(),
                    thickness=HeadingThickness::Thick.into(),
                )
                {
                    "Heading"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.heading.band.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=HeadingVariant::Band.into(),
                            color=color.into(),
                        )
                        {
                            "Heading"
                        }
                    }
                )
            }
        }
    }
}
