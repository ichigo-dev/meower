//------------------------------------------------------------------------------
//! Mypage.
//------------------------------------------------------------------------------

mod index;
mod edit_profile;

pub use index::Index;
pub use edit_profile::EditProfile;

use sycamore::prelude::*;
use sycamore_router::Route;


//------------------------------------------------------------------------------
/// Mypage routes.
//------------------------------------------------------------------------------
#[derive(Route)]
pub enum MypageRoutes
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
pub fn MypageRouter<'cx, G: Html>
(
    cx: Scope<'cx>,
    route: &'cx MypageRoutes,
) -> View<G>
{
    view!
    {
        cx,
        (match route
        {
            MypageRoutes::Index => view! { cx, Index },
            MypageRoutes::EditProfile => view! { cx, EditProfile },
            MypageRoutes::NotFound => view! { cx, "not found" },
        })
    }
}
