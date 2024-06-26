//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct ProfileCoverProps<G: Html>
{
    #[prop(default)]
    pub alt: ReadSignal<String>,

    #[prop(default)]
    pub attributes: Attributes<G>,

    #[prop(default)]
    pub base64: Signal<Option<String>>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub file_key: ReadSignal<Option<String>>,

    #[prop(default)]
    pub node_ref: NodeRef<G>,

    #[prop(default)]
    pub src: ReadSignal<String>,
}
