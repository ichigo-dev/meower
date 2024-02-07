//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ButtonGroupItemProps<G: Html>
{
    #[prop(default)]
    pub active: ReadSignal<bool>,

    #[prop(default)]
    pub attributes: Attributes<G>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default = Colors::Primary.into())]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default)]
    pub href: ReadSignal<Option<String>>,

    #[prop(default)]
    pub left_icon: View<G>,

    #[prop(default)]
    pub right_icon: View<G>,
}
