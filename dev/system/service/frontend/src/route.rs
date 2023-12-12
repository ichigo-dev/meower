//------------------------------------------------------------------------------
//! Application routes.
//------------------------------------------------------------------------------

use sycamore::prelude::*;
use sycamore_router::{ Route, Router, HistoryIntegration };


//------------------------------------------------------------------------------
/// Application routes.
//------------------------------------------------------------------------------
#[derive(Route)]
enum AppRoutes
{
    #[to("/")]
    Top,

    #[to("/mypage")]
    Mypage,

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
                (match route.get().as_ref()
                {
                    AppRoutes::Top => view! { cx, p { "Top" } },
                    AppRoutes::Mypage => view! { cx, p { "Mypage" } },
                    AppRoutes::NotFound => view! { cx, p { "404 Not Found" } },
                })
            }
        )
    }
}
