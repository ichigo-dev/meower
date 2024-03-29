//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use sycamore::prelude::*;

//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ListItemProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub clickable: ReadSignal<bool>,

    #[prop(default)]
    pub divider: ReadSignal<bool>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,
}
