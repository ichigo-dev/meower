//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------

use super::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct AvatarProps<G: Html>
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

    #[prop(default = StrProp("image/png").into())]
    pub mime_type: ReadSignal<String>,

    #[prop(default)]
    pub size: ReadSignal<AvatarSize>,

    #[prop(default)]
    pub src: ReadSignal<String>,
}
