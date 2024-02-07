//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::*;

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

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub right_icon: Option<View<G>>,

    #[prop(default)]
    pub left_icon: Option<View<G>>,

    #[prop(default)]
    pub severity: ReadSignal<AlertSeverity>,

    #[prop(default)]
    pub variant: ReadSignal<AlertVariant>,
}
