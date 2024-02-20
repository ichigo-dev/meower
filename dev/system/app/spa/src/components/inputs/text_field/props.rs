//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::*;
use crate::utils::props::*;
use crate::variables::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct TextFieldProps<G: Html>
{
    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub color: ReadSignal<Colors>,

    #[prop(default)]
    pub disabled: ReadSignal<bool>,

    #[prop(default = StrProp("text").into())]
    pub field_type: ReadSignal<String>,

    #[prop(default)]
    pub full_width: ReadSignal<bool>,

    #[prop(default)]
    pub multiline: ReadSignal<bool>,

    #[prop(default)]
    pub name: ReadSignal<String>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub placeholder: ReadSignal<String>,

    #[prop(default)]
    pub readonly: ReadSignal<bool>,

    #[prop(default)]
    pub required: ReadSignal<bool>,

    #[prop(default = UsizeProp(3).into())]
    pub rows: ReadSignal<usize>,

    #[prop(default)]
    pub size: ReadSignal<TextFieldSize>,

    #[prop(default)]
    pub value: Signal<String>,

    #[prop(default)]
    pub variant: ReadSignal<TextFieldVariant>,
}
