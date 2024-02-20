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
pub struct DrawerProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

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
    pub position: ReadSignal<DrawerPosition>,
}
