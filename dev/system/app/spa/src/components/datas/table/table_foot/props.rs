//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct TableFootProps<G: Html>
{
    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,
}