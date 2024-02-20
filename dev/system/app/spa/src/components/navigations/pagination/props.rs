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
pub struct PaginationProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub count: ReadSignal<isize>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default = IsizeProp(1).into())]
    pub page: Signal<isize>,

    #[prop(default = IsizeProp(10).into())]
    pub per_page: ReadSignal<isize>,

    #[prop(default = BoolProp(true).into())]
    pub show_first_button: ReadSignal<bool>,

    #[prop(default = BoolProp(true).into())]
    pub show_last_button: ReadSignal<bool>,

    #[prop(default = BoolProp(true).into())]
    pub show_next_button: ReadSignal<bool>,

    #[prop(default = BoolProp(true).into())]
    pub show_prev_button: ReadSignal<bool>,

    #[prop(default = IsizeProp(2).into())]
    pub sibling_count: ReadSignal<isize>,

    #[prop(default)]
    pub variant: ReadSignal<PaginationVariant>,
}
