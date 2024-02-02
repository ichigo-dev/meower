//------------------------------------------------------------------------------
//! IconExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn ListExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.icon.heading"))
        }
        MainPanel(children=view!
        {
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                List
                (
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                    }
                )
                List
                (
                    variant=*create_signal("boxed".to_string()),
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                    }
                )
                List
                (
                    variant=*create_signal("simple".to_string()),
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                    }
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                List
                (
                    ordered=*create_signal(true),
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                    }
                )
                List
                (
                    ordered=*create_signal(true),
                    variant=*create_signal("boxed".to_string()),
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                    }
                )
                List
                (
                    ordered=*create_signal(true),
                    variant=*create_signal("simple".to_string()),
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                    }
                )
            }
            Indexed
            (
                iterable=colors,
                view=|color|
                {
                    let cloned_color_1 = color.clone();
                    let cloned_color_2 = color.clone();
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            List
                            (
                                color=*create_signal(cloned_color_1),
                                children=view!
                                {
                                    ListItem
                                    (
                                        clickable=*create_signal(true),
                                        children=view! { "Item 1" }
                                    )
                                    ListItem(children=view! { "Item 2" })
                                    ListItem(children=view! { "Item 3" })
                                }
                            )
                            List
                            (
                                color=*create_signal(cloned_color_2),
                                variant=*create_signal("boxed".to_string()),
                                children=view!
                                {
                                    ListItem
                                    (
                                        clickable=*create_signal(true),
                                        children=view! { "Item 1" }
                                    )
                                    ListItem(children=view! { "Item 2" })
                                    ListItem(children=view! { "Item 3" })
                                }
                            )
                            List
                            (
                                color=*create_signal(color),
                                variant=*create_signal("simple".to_string()),
                                children=view!
                                {
                                    ListItem
                                    (
                                        clickable=*create_signal(true),
                                        children=view! { "Item 1" }
                                    )
                                    ListItem(children=view! { "Item 2" })
                                    ListItem(children=view! { "Item 3" })
                                }
                            )
                        }
                    }
                }
            )
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                List
                (
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                        List
                        (
                            ordered=*create_signal(true),
                            children=view!
                            {
                                ListItem
                                (
                                    clickable=*create_signal(true),
                                    children=view! { "Item 1" }
                                )
                                ListItem(children=view! { "Item 2" })
                                ListItem(children=view! { "Item 3" })
                            }
                        )
                    }
                )
                List
                (
                    variant=*create_signal("boxed".to_string()),
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                        List
                        (
                            ordered=*create_signal(true),
                            children=view!
                            {
                                ListItem
                                (
                                    clickable=*create_signal(true),
                                    children=view! { "Item 1" }
                                )
                                ListItem(children=view! { "Item 2" })
                                ListItem(children=view! { "Item 3" })
                            }
                        )
                    }
                )
                List
                (
                    variant=*create_signal("simple".to_string()),
                    children=view!
                    {
                        ListItem
                        (
                            clickable=*create_signal(true),
                            children=view! { "Item 1" }
                        )
                        ListItem(children=view! { "Item 2" })
                        ListItem(children=view! { "Item 3" })
                        List
                        (
                            ordered=*create_signal(true),
                            children=view!
                            {
                                ListItem
                                (
                                    clickable=*create_signal(true),
                                    children=view! { "Item 1" }
                                )
                                ListItem(children=view! { "Item 2" })
                                ListItem(children=view! { "Item 3" })
                            }
                        )
                    }
                )
            }
        })
    }
}
