//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::*;
use crate::utils::props::*;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct DialogProps<G: Html>
{
    #[prop(default)]
    pub animation: ReadSignal<DialogAnimation>,

    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default = BoolProp(true).into())]
    pub close_on_backdrop: ReadSignal<bool>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub open: Signal<bool>,

    #[prop(default)]
    pub size: ReadSignal<DialogSize>,
}
