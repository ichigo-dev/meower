//------------------------------------------------------------------------------
//! UI catalog page.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::{ Main, MainPanel };

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn UiCatalog<G: Html>() -> View<G>
{
    let theme_colors = create_signal(vec!
    [
        "primary".to_string(),
        "secondary".to_string(),
        "error".to_string(),
        "warning".to_string(),
        "info".to_string(),
        "success".to_string(),
    ]);

    view!
    {
        Main
        (
            heading=t!("pages.dev.ui_catalog.heading_top"),
            children=view!
            {
                BadgeExamples(colors=*theme_colors)
                ChipExamples(colors=*theme_colors)
                HeadingExamples(colors=*theme_colors)
            }
        )
    }
}


//------------------------------------------------------------------------------
/// BadgeExamples.
//------------------------------------------------------------------------------
#[component(inline_props)]
fn BadgeExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.badge.heading"))
        }
        MainPanel(children=view!
        {
            div(class="flex flex_row flex_gap_xl flex_align_center width_full")
            {
                Badge
                (
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
                Badge
                (
                    badge_content=*create_signal("3".to_string()),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
                Badge
                (
                    badge_content=*create_signal("99999".to_string()),
                    max=*create_signal(999),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
                Badge
                (
                    invisible=*create_signal(true),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
                Badge
                (
                    badge_content=*create_signal("0".to_string()),
                    show_zero=*create_signal(true),
                    children=view!
                    {
                        div(class="ui_icon icon_envelope")
                    }
                )
            }
            div(class="flex flex_row flex_gap_xl flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Badge
                        (
                            badge_content=*create_signal("3".to_string()),
                            color=*create_signal(color),
                            children=view!
                            {
                                div(class="ui_icon icon_envelope")
                            }
                        )
                    }
                )
            }
        })
    }
}


//------------------------------------------------------------------------------
/// ChipExamples.
//------------------------------------------------------------------------------
#[component(inline_props)]
fn ChipExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.chip.heading"))
        }
        MainPanel(children=view!
        {
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string())
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    disabled=*create_signal(true),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                    disabled=*create_signal(true),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    clickable=*create_signal(true),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                    clickable=*create_signal(true),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    left_icon=view! { span(class="ui_icon icon_envelope") }
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                    right_icon=view!
                    {
                        span(class="ui_icon icon_xmark clickable")
                    }
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    size=*create_signal("small".to_string()),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    size=*create_signal("large".to_string()),
                )
            }
            Indexed
            (
                iterable=colors,
                view=|color|
                {
                    let cloned_color = color.clone();
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Chip
                            (
                                label=*create_signal("Chip".to_string()),
                                color=*create_signal(cloned_color),
                            )
                            Chip
                            (
                                label=*create_signal("Chip".to_string()),
                                color=*create_signal(color),
                                variant=*create_signal("outlined".to_string()),
                            )
                        }
                    }
                }
            )
        })
    }
}


//------------------------------------------------------------------------------
/// HeadingExamples.
//------------------------------------------------------------------------------
#[component(inline_props)]
fn HeadingExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.heading.heading"))
        }
        MainPanel(children=view!
        {
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Heading
                (
                    level=*create_signal("h1".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    level=*create_signal("h2".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    level=*create_signal("h3".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    level=*create_signal("h4".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    level=*create_signal("h5".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    level=*create_signal("h6".to_string()),
                    children=view! { "Heading" }
                )
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    align=*create_signal("left".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    align=*create_signal("center".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    align=*create_signal("right".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    thickness=*create_signal("thin".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("divider".to_string()),
                    thickness=*create_signal("thick".to_string()),
                    children=view! { "Heading" }
                )
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=*create_signal("divider".to_string()),
                            color=*create_signal(color),
                            children=view! { "Heading" }
                        )
                    }
                )
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=*create_signal("bullet".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("bullet".to_string()),
                    thickness=*create_signal("thin".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("bullet".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("bullet".to_string()),
                    thickness=*create_signal("thick".to_string()),
                    children=view! { "Heading" }
                )
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=*create_signal("bullet".to_string()),
                            color=*create_signal(color),
                            children=view! { "Heading" }
                        )
                    }
                )
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    align=*create_signal("left".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    align=*create_signal("center".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    align=*create_signal("right".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    thickness=*create_signal("thin".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("line".to_string()),
                    thickness=*create_signal("thick".to_string()),
                    children=view! { "Heading" }
                )
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=*create_signal("line".to_string()),
                            color=*create_signal(color),
                            children=view! { "Heading" }
                        )
                    }
                )
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    align=*create_signal("left".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    align=*create_signal("center".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    align=*create_signal("right".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    thickness=*create_signal("thin".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    children=view! { "Heading" }
                )
                Heading
                (
                    variant=*create_signal("band".to_string()),
                    thickness=*create_signal("thick".to_string()),
                    children=view! { "Heading" }
                )
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Heading
                        (
                            variant=*create_signal("band".to_string()),
                            color=*create_signal(color),
                            children=view! { "Heading" }
                        )
                    }
                )
            }
        })
    }
}
