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
pub struct RangeSliderProps<G: Html>
{
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default)]
    pub name: ReadSignal<String>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub size: ReadSignal<RangeSliderSize>,

    #[prop(default)]
    pub value: ReadSignal<String>,
}
