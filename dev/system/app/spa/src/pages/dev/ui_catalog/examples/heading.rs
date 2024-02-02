//------------------------------------------------------------------------------
//! HeadingExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn HeadingExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
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
