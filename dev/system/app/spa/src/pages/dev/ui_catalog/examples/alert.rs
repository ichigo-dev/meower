//------------------------------------------------------------------------------
//! AlertExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn AlertExamples<G: Html>() -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.alert.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.alert.basic.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Alert(severity=AlertSeverity::Error.into())
                {
                    "Alert"
                }
                Alert(severity=AlertSeverity::Warning.into())
                {
                    "Alert"
                }
                Alert(severity=AlertSeverity::Info.into())
                {
                    "Alert"
                }
                Alert(severity=AlertSeverity::Success.into())
                {
                    "Alert"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.alert.outlined.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Alert
                (
                    severity=AlertSeverity::Error.into(),
                    variant=AlertVariant::Outlined.into(),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=AlertSeverity::Warning.into(),
                    variant=AlertVariant::Outlined.into(),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=AlertSeverity::Info.into(),
                    variant=AlertVariant::Outlined.into(),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=AlertSeverity::Success.into(),
                    variant=AlertVariant::Outlined.into(),
                )
                {
                    "Alert"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.alert.filled.heading"))
            }
            div(class="flex flex_column flex_gap_md width_full")
            {
                Alert
                (
                    severity=AlertSeverity::Error.into(),
                    variant=AlertVariant::Filled.into(),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=AlertSeverity::Warning.into(),
                    variant=AlertVariant::Filled.into(),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=AlertSeverity::Info.into(),
                    variant=AlertVariant::Filled.into(),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=AlertSeverity::Success.into(),
                    variant=AlertVariant::Filled.into(),
                )
                {
                    "Alert"
                }
            }
        }
    }
}
