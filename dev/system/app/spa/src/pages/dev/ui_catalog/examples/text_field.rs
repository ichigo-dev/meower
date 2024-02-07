//------------------------------------------------------------------------------
//! TextFieldExamples.
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
pub fn TextFieldExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.text_field.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.text_field.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Filled.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Outlined.into(),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Password").into(),
                    field_type=StringProp("password").into(),
                )
                TextField
                (
                    placeholder=StringProp("Number").into(),
                    field_type=StringProp("number").into(),
                )
                TextField(field_type=StringProp("date").into())
                TextField(field_type=StringProp("time").into())
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.text_field.disabled.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                    disabled=BoolProp(true).into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Filled.into(),
                    disabled=BoolProp(true).into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Outlined.into(),
                    disabled=BoolProp(true).into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.text_field.readonly.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                    readonly=BoolProp(true).into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Filled.into(),
                    readonly=BoolProp(true).into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Outlined.into(),
                    readonly=BoolProp(true).into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.text_field.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                    size=TextFieldSize::Small.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                    size=TextFieldSize::Medium.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                    size=TextFieldSize::Large.into(),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Filled.into(),
                    size=TextFieldSize::Small.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Filled.into(),
                    size=TextFieldSize::Medium.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Filled.into(),
                    size=TextFieldSize::Large.into(),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Outlined.into(),
                    size=TextFieldSize::Small.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Outlined.into(),
                    size=TextFieldSize::Medium.into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Outlined.into(),
                    size=TextFieldSize::Large.into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.text_field.full_width.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                    readonly=BoolProp(true).into(),
                    full_width=BoolProp(true).into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.text_field.multiline.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Default.into(),
                    multiline=BoolProp(true).into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Filled.into(),
                    multiline=BoolProp(true).into(),
                )
                TextField
                (
                    placeholder=StringProp("Text Field").into(),
                    variant=TextFieldVariant::Outlined.into(),
                    multiline=BoolProp(true).into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.text_field.multiline.heading"))
            }
            Indexed
            (
                iterable=colors,
                view=|color| view!
                {
                    div(class="flex flex_row flex_gap_md flex_align_center width_full")
                    {
                        TextField
                        (
                            color=color.into(),
                            placeholder=StringProp("Text Field").into(),
                            variant=TextFieldVariant::Default.into(),
                        )
                        TextField
                        (
                            color=color.into(),
                            placeholder=StringProp("Text Field").into(),
                            variant=TextFieldVariant::Filled.into(),
                        )
                        TextField
                        (
                            color=color.into(),
                            placeholder=StringProp("Text Field").into(),
                            variant=TextFieldVariant::Outlined.into(),
                        )
                    }
                }
            )
        }
    }
}
