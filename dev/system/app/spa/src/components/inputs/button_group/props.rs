//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::*;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ButtonGroupProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default = Colors::Primary.into())]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub size: ReadSignal<ButtonGroupSize>,

    #[prop(default)]
    pub variant: ReadSignal<ButtonGroupVariant>,

    #[prop(default)]
    pub vertical: ReadSignal<bool>,
}
