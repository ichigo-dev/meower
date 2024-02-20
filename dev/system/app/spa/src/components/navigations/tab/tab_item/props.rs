//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use crate::components::TabValue;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct TabItemProps<G: Html>
{
    #[prop(default)]
    pub active: Signal<bool>,

    #[prop(default)]
    pub attributes: Attributes<G>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub value: ReadSignal<TabValue>,
}
