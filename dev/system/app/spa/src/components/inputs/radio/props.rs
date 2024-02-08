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
pub struct RadioProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub checked: Signal<bool>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default)]
    pub name: ReadSignal<String>,

    #[prop(default)]
    pub required: ReadSignal<bool>,

    #[prop(default)]
    pub size: ReadSignal<RadioSize>,

    #[prop(default)]
    pub value: ReadSignal<String>,
}
