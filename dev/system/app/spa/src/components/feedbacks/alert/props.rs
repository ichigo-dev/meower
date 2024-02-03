//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::severity::AlertSeverity;
use super::variant::AlertVariant;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct AlertProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub severity: ReadSignal<AlertSeverity>,

    #[prop(default)]
    pub variant: ReadSignal<AlertVariant>,
}
