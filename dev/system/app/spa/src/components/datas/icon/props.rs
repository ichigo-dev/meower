//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::kind::IconKind;
use super::size::IconSize;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct IconProps
{
    pub icon: ReadSignal<IconKind>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub size: ReadSignal<IconSize>,
}
