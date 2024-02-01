//------------------------------------------------------------------------------
//! Account routes.
//------------------------------------------------------------------------------

use super::*;
use crate::pages::notfound::NotFound;

use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::Route;


//------------------------------------------------------------------------------
/// Routes.
//------------------------------------------------------------------------------
#[derive(Route, Clone)]
pub enum Routes
{
    #[to("/")]
    Index,

    #[to("/create")]
    Create,

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Router.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Router<'cx, G: Html>( cx: Scope<'cx>, route: Routes ) -> View<G>
{
    view!
    {
        cx,
        (match route
        {
            Routes::Index =>
            {
                view!
                {
                    cx,
                    Suspense(fallback=view! { cx, "Loading..." }) { Index }
                }
            },
            Routes::Create =>
            {
                view!
                {
                    cx,
                    Suspense(fallback=view! { cx, "Loading..." }) { Create }
                }
            },
            Routes::NotFound => view! { cx, NotFound },
        })
    }
}
