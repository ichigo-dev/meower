//------------------------------------------------------------------------------
//! Layout.
//------------------------------------------------------------------------------

use super::{ Header, Footer, Aside };
use crate::components::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Layout<G: Html>( children: Children<G> ) -> View<G>
{
    let children = children.call();
    view!
    {
        Box(classes=StrProp("flex flex_column min_height_full_viewport").into())
        {
            Box(classes=StrProp("theme_meower_light flex_grow flex flex_row").into())
            {
                Aside()
                Box(classes=StrProp("flex_grow flex flex_column overflow_auto_x").into())
                {
                    Header()
                    Box(classes=StrProp("flex_grow padding_xl").into())
                    {
                        (children)
                    }
                    Footer()
                }
            }
        }
    }
}
