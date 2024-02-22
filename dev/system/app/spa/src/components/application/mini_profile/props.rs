//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct MiniProfileProps<G: Html>
{
    #[prop(default)]
    pub account_name: String,

    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub clickable: ReadSignal<bool>,

    #[prop(default)]
    pub file_key: Option<String>,

    #[prop(default)]
    pub name: String,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub selected: ReadSignal<bool>,
}
