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
pub struct TypographyProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    pub children: Children<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub font_size: ReadSignal<TypographyFontSize>,

    #[prop(default)]
    pub font_weight: ReadSignal<TypographyFontWeight>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,
}
