//------------------------------------------------------------------------------
//! Dev routes.
//------------------------------------------------------------------------------

use super::*;
use crate::pages::notfound::NotFound;

use sycamore::prelude::*;
use sycamore_router::Route;


//------------------------------------------------------------------------------
/// Routes.
//------------------------------------------------------------------------------
#[derive(Clone, Route)]
pub enum Routes
{
    #[to("/")]
    Index,

    #[to("/ui_catalog")]
    UiCatalog,

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Router.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Router<G: Html>( route: Routes ) -> View<G>
{
    view!
    {
        (match route
        {
            Routes::Index => view! { Index },
            Routes::UiCatalog => view! { UiCatalog },
            Routes::NotFound => view! { NotFound },
        })
    }
}
