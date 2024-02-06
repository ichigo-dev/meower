//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------

use crate::utils::props::*;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct BadgeProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub badge_content: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub invisible: ReadSignal<bool>,

    #[prop(default = UsizeProp(0).into())]
    pub max: ReadSignal<usize>,

    #[prop(default)]
    pub show_zero: ReadSignal<bool>,
}
