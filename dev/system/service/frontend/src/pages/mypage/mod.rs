//------------------------------------------------------------------------------
//! Mypage.
//------------------------------------------------------------------------------

mod index;

pub use index::Index;

use sycamore::prelude::*;
use sycamore_router::{ Route, Router, HistoryIntegration };


//------------------------------------------------------------------------------
/// Mypage routes.
//------------------------------------------------------------------------------
#[derive(Route)]
pub enum MypageRoutes
{
    #[to("/")]
    Index,

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Mypage router.
//------------------------------------------------------------------------------
#[component]
pub fn MypageRouter<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        Router
        (
            integration=HistoryIntegration::new(),
            view=move |cx, route: &ReadSignal<MypageRoutes>|
            {
                match route.get().as_ref()
                {
                    MypageRoutes::Index => view! { cx, Index },
                    MypageRoutes::NotFound => view! { cx, div { "not found" } },
                }
            }
        )
    }
}
