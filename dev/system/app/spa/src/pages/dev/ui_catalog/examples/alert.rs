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
                Alert(severity=*create_signal("error".to_string()))
                {
                    "Alert"
                }
                Alert(severity=*create_signal("warning".to_string()))
                {
                    "Alert"
                }
                Alert(severity=*create_signal("info".to_string()))
                {
                    "Alert"
                }
                Alert(severity=*create_signal("success".to_string()))
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
                    severity=*create_signal("error".to_string()),
                    variant=*create_signal("outlined".to_string()),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=*create_signal("warning".to_string()),
                    variant=*create_signal("outlined".to_string()),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=*create_signal("info".to_string()),
                    variant=*create_signal("outlined".to_string()),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=*create_signal("success".to_string()),
                    variant=*create_signal("outlined".to_string()),
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
                    severity=*create_signal("error".to_string()),
                    variant=*create_signal("filled".to_string()),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=*create_signal("warning".to_string()),
                    variant=*create_signal("filled".to_string()),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=*create_signal("info".to_string()),
                    variant=*create_signal("filled".to_string()),
                )
                {
                    "Alert"
                }
                Alert
                (
                    severity=*create_signal("success".to_string()),
                    variant=*create_signal("filled".to_string()),
                )
                {
                    "Alert"
                }
            }
        }
    }
}
