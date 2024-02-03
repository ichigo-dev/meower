//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::align::HeadingAlign;
use super::level::HeadingLevel;
use super::thickness::HeadingThickness;
use super::variant::HeadingVariant;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct HeadingProps<G: Html>
{
    #[prop(default)]
    pub align: ReadSignal<HeadingAlign>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub level: ReadSignal<HeadingLevel>,

    #[prop(default)]
    pub thickness: ReadSignal<HeadingThickness>,

    #[prop(default)]
    pub variant: ReadSignal<HeadingVariant>,
}
