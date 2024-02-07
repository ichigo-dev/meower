//------------------------------------------------------------------------------
//! PaginationExamples.
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
pub fn PaginationExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.pagination.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.pagination.basic.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Pagination
                (
                    count=IsizeProp(100).into(),
                    variant=PaginationVariant::Circle.into(),
                )
                Pagination
                (
                    count=IsizeProp(100).into(),
                    variant=PaginationVariant::CircleOutlined.into(),
                )
                Pagination
                (
                    count=IsizeProp(100).into(),
                    variant=PaginationVariant::Rounded.into(),
                )
                Pagination
                (
                    count=IsizeProp(100).into(),
                    variant=PaginationVariant::RoundedOutlined.into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.pagination.colors.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Pagination
                        (
                            color=color.into(),
                            count=IsizeProp(100).into(),
                            variant=PaginationVariant::Circle.into(),
                        )
                        Pagination
                        (
                            color=color.into(),
                            count=IsizeProp(100).into(),
                            variant=PaginationVariant::CircleOutlined.into(),
                        )
                        Pagination
                        (
                            color=color.into(),
                            count=IsizeProp(100).into(),
                            variant=PaginationVariant::Rounded.into(),
                        )
                        Pagination
                        (
                            color=color.into(),
                            count=IsizeProp(100).into(),
                            variant=PaginationVariant::RoundedOutlined.into(),
                        )
                    },
                )
            }
        }
    }
}
