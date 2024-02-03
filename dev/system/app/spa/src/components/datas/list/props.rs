//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::variant::ListVariant;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ListProps<G: Html>
{
    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub ordered: ReadSignal<bool>,

    #[prop(default)]
    pub variant: ReadSignal<ListVariant>,
}
