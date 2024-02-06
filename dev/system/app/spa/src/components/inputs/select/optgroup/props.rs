//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct OptgroupProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default)]
    pub name: ReadSignal<String>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub value: ReadSignal<String>,
}
