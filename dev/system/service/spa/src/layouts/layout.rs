//------------------------------------------------------------------------------
//! Layout.
//------------------------------------------------------------------------------

use super::{ Header, Footer, Aside };

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Layout<G: Html>( cx: Scope, child: View<G> ) -> View<G>
{
    view!
    {
        cx,
        div(class="flex flex_column min_height_full_viewport")
        {
            div(class="theme_meower_light flex_grow flex flex_row")
            {
                Aside()
                div(class="flex_grow flex flex_column")
                {
                    Header()
                    div(class="flex_grow padding_xl")
                    {
                        (child)
                    }
                    Footer()
                }
            }
        }
    }
}
