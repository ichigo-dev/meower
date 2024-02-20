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
pub struct LabelProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub direction: ReadSignal<LabelDirection>,

    #[prop(default)]
    pub label: String,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub required: ReadSignal<bool>,
}
