//------------------------------------------------------------------------------
//! TableExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn TableExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    let iter = create_signal((1..=5).collect::<Vec<_>>());
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.table.heading"))
        }
        MainPanel
        {
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Table
                {
                    TableHead
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Header " (index)
                                    }
                                }
                            )
                        }
                    }
                    TableBody
                    {
                        Indexed
                        (
                            iterable=*iter,
                            view=move |i| view!
                            {
                                TableRow
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |j| view!
                                        {
                                            TableCell
                                            {
                                                "Data " (i) "-" (j)
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    }
                    TableFoot
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Footer " (index)
                                    }
                                }
                            )
                        }
                    }
                }
                Table
                {
                    TableBody
                    {
                        Indexed
                        (
                            iterable=*iter,
                            view=move |i| view!
                            {
                                TableRow
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Header " (i)
                                    }
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |j| view!
                                        {
                                            TableCell
                                            {
                                                "Data " (i) "-" (j)
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    }
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Table(size=*create_signal("small".to_string()))
                {
                    TableHead
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Header " (index)
                                    }
                                }
                            )
                        }
                    }
                    TableBody
                    {
                        Indexed
                        (
                            iterable=*iter,
                            view=move |i| view!
                            {
                                TableRow
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |j| view!
                                        {
                                            TableCell
                                            {
                                                "Data " (i) "-" (j)
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    }
                }
                Table
                {
                    TableHead
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Header " (index)
                                    }
                                }
                            )
                        }
                    }
                    TableBody
                    {
                        Indexed
                        (
                            iterable=*iter,
                            view=move |i| view!
                            {
                                TableRow
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |j| view!
                                        {
                                            TableCell
                                            {
                                                "Data " (i) "-" (j)
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    }
                }
                Table(size=*create_signal("large".to_string()))
                {
                    TableHead
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Header " (index)
                                    }
                                }
                            )
                        }
                    }
                    TableBody
                    {
                        Indexed
                        (
                            iterable=*iter,
                            view=move |i| view!
                            {
                                TableRow
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |j| view!
                                        {
                                            TableCell
                                            {
                                                "Data " (i) "-" (j)
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    }
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Table(variant=*create_signal("stripe".to_string()))
                {
                    TableHead
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Header " (index)
                                    }
                                }
                            )
                        }
                    }
                    TableBody
                    {
                        Indexed
                        (
                            iterable=*iter,
                            view=move |i| view!
                            {
                                TableRow
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |j| view!
                                        {
                                            TableCell
                                            {
                                                "Data " (i) "-" (j)
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    }
                }
                Table(variant=*create_signal("tiled".to_string()))
                {
                    TableHead
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=*create_signal(true))
                                    {
                                        "Header " (index)
                                    }
                                }
                            )
                        }
                    }
                    TableBody
                    {
                        Indexed
                        (
                            iterable=*iter,
                            view=move |i| view!
                            {
                                TableRow
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |j| view!
                                        {
                                            TableCell
                                            {
                                                "Data " (i) "-" (j)
                                            }
                                        }
                                    )
                                }
                            }
                        )
                    }
                }
            }
            Indexed
            (
                iterable=colors,
                view=move |color|
                {
                    let cloned_color = color.clone();
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Table(color=*create_signal(cloned_color))
                            {
                                TableHead
                                {
                                    TableRow
                                    {
                                        Indexed
                                        (
                                            iterable=*iter,
                                            view=|index| view!
                                            {
                                                TableCell(is_head=*create_signal(true))
                                                {
                                                    "Header " (index)
                                                }
                                            }
                                        )
                                    }
                                }
                                TableBody
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |i| view!
                                        {
                                            TableRow
                                            {
                                                Indexed
                                                (
                                                    iterable=*iter,
                                                    view=move |j| view!
                                                    {
                                                        TableCell
                                                        {
                                                            "Data " (i) "-" (j)
                                                        }
                                                    }
                                                )
                                            }
                                        }
                                    )
                                }
                            }
                            Table(color=*create_signal(color))
                            {
                                TableBody
                                {
                                    Indexed
                                    (
                                        iterable=*iter,
                                        view=move |i| view!
                                        {
                                            TableRow
                                            {
                                                TableCell(is_head=*create_signal(true))
                                                {
                                                    "Header " (i)
                                                }
                                                Indexed
                                                (
                                                    iterable=*iter,
                                                    view=move |j| view!
                                                    {
                                                        TableCell
                                                        {
                                                            "Data " (i) "-" (j)
                                                        }
                                                    }
                                                )
                                            }
                                        }
                                    )
                                }
                            }
                        }
                    }
                }
            )
        }
    }
}
