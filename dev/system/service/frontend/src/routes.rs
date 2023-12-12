//------------------------------------------------------------------------------
//! Application routes.
//------------------------------------------------------------------------------

use crate::pages::*;

use sycamore::prelude::*;
use sycamore_router::{ Route, Router, HistoryIntegration };


//------------------------------------------------------------------------------
/// Application routes.
//------------------------------------------------------------------------------

// Base routes.
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

// Mypage routes.
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
                    AppRoutes::Mypage(mypage_route) =>
                    {
                        match mypage_route
                        {
                            MypageRoutes::Index => view! { cx, mypage::Index },
                            MypageRoutes::EditProfile =>
                            {
                                view! { cx, mypage::EditProfile }
                            },
                            MypageRoutes::NotFound => view! { cx, NotFound },
                        }
                    },
                    AppRoutes::NotFound => view! { cx, NotFound },
                }
            }
        )
    }
}
