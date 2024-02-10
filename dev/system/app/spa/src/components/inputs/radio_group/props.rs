//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct RadioGroupProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub name: ReadSignal<String>,

    #[prop(default)]
    pub value: Signal<RadioValue>,
}
