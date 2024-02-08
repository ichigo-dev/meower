//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct FormProps<G: Html>
{
    #[prop(default)]
    pub action: ReadSignal<String>,

    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default = StrProp("POST").into())]
    pub method: ReadSignal<String>,

    #[prop(default)]
    pub submit: ReadSignal<bool>,

    #[prop(default)]
    pub values: Signal<FormValues>,
}
