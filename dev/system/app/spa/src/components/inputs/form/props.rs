//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::*;
use crate::utils::props::*;

use sycamore::prelude::*;
use web_sys::SubmitEvent;


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
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub on_submit: Option<Box<dyn Fn(FormValues, SubmitEvent) -> ()>>,

    #[prop(default)]
    pub submit: ReadSignal<bool>,

    #[prop(default)]
    pub values: Signal<FormValues>,
}
