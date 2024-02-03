//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::size::ProgressSize;
use super::thickness::ProgressThickness;
use super::variant::ProgressVariant;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ProgressProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    pub children: Children<G>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub size: ReadSignal<ProgressSize>,
 
    #[prop(default)]
    pub thickness: ReadSignal<ProgressThickness>,

    #[prop(default)]
    pub variant: ReadSignal<ProgressVariant>,
}
