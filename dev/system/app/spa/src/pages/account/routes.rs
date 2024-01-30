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
pub(crate) enum Routes
{
    #[to("/")]
    Index,

    #[to("/edit_account")]
    EditAccount,

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Router.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub(crate) fn Router<'cx, G: Html>( cx: Scope<'cx>, route: Routes ) -> View<G>
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
            Routes::EditAccount => view! { cx, EditAccount },
            Routes::NotFound => view! { cx, NotFound },
        })
    }
}
