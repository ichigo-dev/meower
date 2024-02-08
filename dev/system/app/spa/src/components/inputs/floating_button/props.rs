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
pub struct FloatingButtonProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default = StrProp("button").into())]
    pub button_type: ReadSignal<String>,

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
    pub icon: View<G>,

    #[prop(default)]
    pub name: ReadSignal<String>,
 
    #[prop(default)]
    pub size: ReadSignal<FloatingButtonSize>,

    #[prop(default)]
    pub value: Signal<String>,
}
