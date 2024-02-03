//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::animation::SnackbarAnimation;
use super::position::SnackbarPosition;
use crate::variables::Colors;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct SnackbarProps<G: Html>
{
    #[prop(default)]
    pub animation: ReadSignal<SnackbarAnimation>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default = Colors::Background.into())]
    pub close_icon_color: ReadSignal<Colors>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub open: Signal<bool>,

    #[prop(default)]
    pub position: ReadSignal<SnackbarPosition>,

    #[prop(default)]
    pub show_close_icon: ReadSignal<bool>,
}
