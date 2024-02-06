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
pub struct IconProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    pub icon: ReadSignal<IconKind>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub clickable: ReadSignal<bool>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub size: ReadSignal<IconSize>,
}
