//------------------------------------------------------------------------------
//! NotFound page.
//------------------------------------------------------------------------------

use crate::layouts::application::Layout;
use crate::components::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn NotFound<G: Html>() -> View<G>
{
    view!
    {
        Layout
        {
            Heading(level=HeadingLevel::H1.into())
            {
                (t!("pages.notfound.heading"))
            }
        }
    }
}
