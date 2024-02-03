//------------------------------------------------------------------------------
//! ListExamples.
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
pub fn ListExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.list.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.list.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                List
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                }
                List(variant=ListVariant::Boxed.into())
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                }
                List(variant=ListVariant::Simple.into())
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                List(ordered=BoolProp(true).into())
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                }
                List
                (
                    ordered=BoolProp(true).into(),
                    variant=ListVariant::Boxed.into(),
                )
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                }
                List
                (
                    ordered=BoolProp(true).into(),
                    variant=ListVariant::Simple.into(),
                )
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                }

            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.list.colors.heading"))
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
                            List(color=color.into())
                            {
                                ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                                ListItem { "Item 2" }
                                ListItem { "Item 3" }
                            }
                            List
                            (
                                color=color.into(),
                                variant=ListVariant::Boxed.into(),
                            )
                            {
                                ListItem(clickable=BoolProp(true).into())
                                {
                                    "Item 1"
                                }
                                ListItem { "Item 2" }
                                ListItem { "Item 3" }
                            }
                            List
                            (
                                color=color.into(),
                                variant=ListVariant::Simple.into(),
                            )
                            {
                                ListItem(clickable=BoolProp(true).into())
                                {
                                    "Item 1"
                                }
                                ListItem { "Item 2" }
                                ListItem { "Item 3" }
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
                (t!("pages.dev.ui_catalog.list.nested.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                List
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                    List(ordered=BoolProp(true).into())
                    {
                        ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                        ListItem { "Item 2" }
                        ListItem { "Item 3" }
                    }
                }
                List(variant=ListVariant::Simple.into())
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                    List(ordered=BoolProp(true).into())
                    {
                        ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                        ListItem { "Item 2" }
                        ListItem { "Item 3" }
                    }
                }
                List(variant=ListVariant::Boxed.into())
                {
                    ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                    ListItem { "Item 2" }
                    ListItem { "Item 3" }
                    List(ordered=BoolProp(true).into())
                    {
                        ListItem(clickable=BoolProp(true).into()) { "Item 1" }
                        ListItem { "Item 2" }
                        ListItem { "Item 3" }
                    }
                }
            }
        }
    }
}
