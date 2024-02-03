//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::size::TableSize;
use super::variant::TableVariant;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct TableProps<G: Html>
{
    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub size: ReadSignal<TableSize>,

    #[prop(default)]
    pub sticky: ReadSignal<bool>,

    #[prop(default)]
    pub variant: ReadSignal<TableVariant>,
}