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
                Heading(level=*create_signal("h1".to_string())) { "Heading" }
                Heading(level=*create_signal("h2".to_string())) { "Heading" }
                Heading(level=*create_signal("h3".to_string())) { "Heading" }
                Heading(level=*create_signal("h4".to_string())) { "Heading" }
                Heading(level=*create_signal("h5".to_string())) { "Heading" }
                Heading(level=*create_signal("h6".to_string())) { "Heading" }
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
                    variant=*create_signal("divider".to_string()),
                    align=*create_signal("left".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    align=*create_signal("center".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    align=*create_signal("right".to_string()),
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
                    variant=*create_signal("divider".to_string()),
                    thickness=*create_signal("thin".to_string()),
                )
                {
                    "Heading"
                }
                Heading(variant=*create_signal("divider".to_string()))
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    thickness=*create_signal("thick".to_string()),
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
                            variant=*create_signal("divider".to_string()),
                            color=*create_signal(color),
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
                    variant=*create_signal("bullet".to_string()),
                    thickness=*create_signal("thin".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("bullet".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("bullet".to_string()),
                    thickness=*create_signal("thick".to_string()),
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
                            variant=*create_signal("bullet".to_string()),
                            color=*create_signal(color),
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
                    variant=*create_signal("line".to_string()),
                    align=*create_signal("left".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    align=*create_signal("center".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    align=*create_signal("right".to_string()),
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
                    variant=*create_signal("line".to_string()),
                    thickness=*create_signal("thin".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("line".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    thickness=*create_signal("thick".to_string()),
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
                            variant=*create_signal("line".to_string()),
                            color=*create_signal(color),
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
                    variant=*create_signal("band".to_string()),
                    align=*create_signal("left".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    align=*create_signal("center".to_string()),
                )
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    align=*create_signal("right".to_string()),
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
                    variant=*create_signal("band".to_string()),
                    thickness=*create_signal("thin".to_string()),
                )
                {
                    "Heading"
                }
                Heading(variant=*create_signal("band".to_string()))
                {
                    "Heading"
                }
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    thickness=*create_signal("thick".to_string()),
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
                            variant=*create_signal("band".to_string()),
                            color=*create_signal(color),
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
