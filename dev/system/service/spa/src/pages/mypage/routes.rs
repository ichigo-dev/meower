//------------------------------------------------------------------------------
//! Mypage routes.
//------------------------------------------------------------------------------

use super::*;
use crate::pages::notfound::NotFound;

use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::Route;


//------------------------------------------------------------------------------
/// Mypage routes.
//------------------------------------------------------------------------------
#[derive(Route, Clone)]
pub(crate) enum MypageRoutes
{
    #[to("/")]
    Index,

    #[to("/edit_profile")]
    EditProfile,

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Mypage router.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub(crate) fn MypageRouter<'cx, G: Html>
(
    cx: Scope<'cx>,
    route: MypageRoutes,
) -> View<G>
{
    view!
    {
        cx,
        (match route
        {
            MypageRoutes::Index =>
            {
                view!
                {
                    cx,
                    Suspense(fallback=view! { cx, "Loading..." }) { index::Index }
                }
            },
            MypageRoutes::EditProfile => view! { cx, edit_profile::EditProfile },
            MypageRoutes::NotFound => view! { cx, NotFound },
        })
    }
}
