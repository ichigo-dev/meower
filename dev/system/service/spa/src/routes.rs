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
    Mypage(mypage::MypageRoutes),

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
                let route = route.get();
                match route.as_ref()
                {
                    AppRoutes::Home => view! { cx, home::Home },
                    AppRoutes::Mypage(mypage_route) =>
                    {
                        view!{ cx, mypage::MypageRouter(route=mypage_route.clone()) }
                    },
                    AppRoutes::NotFound => view! { cx, notfound::NotFound },
                }
            }
        )
    }
}
