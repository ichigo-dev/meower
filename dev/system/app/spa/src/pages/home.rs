//------------------------------------------------------------------------------
//! Home page.
//------------------------------------------------------------------------------

use crate::components::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Home<G: Html>() -> View<G>
{
    view!
    {
        Heading(level=HeadingLevel::H1.into())
        {
            (t!("pages.home.heading"))
        }
    }
}
