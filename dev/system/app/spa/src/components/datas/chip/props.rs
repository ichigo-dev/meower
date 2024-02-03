//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------

use super::size::ChipSize;
use super::variant::ChipVariant;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ChipProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub clickable: ReadSignal<bool>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default)]
    pub label: ReadSignal<String>,

    #[prop(default)]
    pub left_icon: View<G>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub right_icon: View<G>,

    #[prop(default)]
    pub size: ReadSignal<ChipSize>,

    #[prop(default)]
    pub variant: ReadSignal<ChipVariant>,
}
