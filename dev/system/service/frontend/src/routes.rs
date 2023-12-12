//------------------------------------------------------------------------------
//! Application routes.
//------------------------------------------------------------------------------

use crate::pages::*;

use sycamore::prelude::*;
use sycamore_router::{ Route, Router, HistoryIntegration };


//------------------------------------------------------------------------------
/// Application routes.
//------------------------------------------------------------------------------
#[derive(Route)]
enum AppRoutes
{
    #[to("/")]
    Home,

    #[to("/mypage/<_..>")]
    Mypage(MypageRoutes),

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Application router.
//------------------------------------------------------------------------------
#[component]
pub fn AppRouter<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        Router
        (
            integration=HistoryIntegration::new(),
            view=move |cx, route: &ReadSignal<AppRoutes>|
            {
                match route.get().as_ref()
                {
                    AppRoutes::Home => view! { cx, Home },
                    AppRoutes::Mypage(_) => view! { cx, MypageRouter },
                    AppRoutes::NotFound => view! { cx, NotFound },
                }
            }
        )
    }
}
