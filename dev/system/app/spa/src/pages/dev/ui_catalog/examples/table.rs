//------------------------------------------------------------------------------
//! TableExamples.
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
pub fn TableExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    let iter = create_signal((1..=5).collect::<Vec<_>>());
    let iter_large = create_signal((1..=20).collect::<Vec<_>>());
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.table.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.table.basic.heading"))
            }
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
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.table.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Table(size=TableSize::Small.into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                    TableHead
                    {
                        TableRow
                        {
                            Indexed
                            (
                                iterable=*iter,
                                view=|index| view!
                                {
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
                                    {
                                        "Footer " (index)
                                    }
                                }
                            )
                        }
                    }
                }
                Table(size=TableSize::Large.into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
                                    {
                                        "Footer " (index)
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.table.stripe.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Table(variant=TableVariant::Stripe.into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
                                    {
                                        "Footer " (index)
                                    }
                                }
                            )
                        }
                    }
                }
                Table(variant=TableVariant::StripeVertical.into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
                                    {
                                        "Footer " (index)
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.table.tiled.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Table(variant=TableVariant::Tiled.into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
                                    {
                                        "Footer " (index)
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.table.colors.heading"))
            }
            Indexed
            (
                iterable=colors,
                view=move |color|
                {
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Table(color=color.into())
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
                                                TableCell(is_head=BoolProp(true).into())
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
                            Table(color=color.into())
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
                                                TableCell(is_head=BoolProp(true).into())
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
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.table.sticky.heading"))
            }
            div(class="flex flex_row flex_gap_md overflow_auto max_height_8xl")
            {
                Table(sticky=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
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
                            iterable=*iter_large,
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
                                    TableCell(is_head=BoolProp(true).into())
                                    {
                                        "Footer " (index)
                                    }
                                }
                            )
                        }
                    }
                }
            }
            div(class="flex flex_row flex_gap_md overflow_auto max_width_10xl")
            {
                Table(sticky=BoolProp(true).into())
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
                                    TableCell(is_head=BoolProp(true).into())
                                    {
                                        "Header " (i)
                                    }
                                    Indexed
                                    (
                                        iterable=*iter_large,
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
}
