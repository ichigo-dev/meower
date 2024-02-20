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
pub struct TooltipProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    pub description: View<G>,

    #[prop(default)]
    pub position: ReadSignal<TooltipPosition>,

    #[prop(default = UsizeProp(250).into())]
    pub max_width: ReadSignal<usize>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,
}
