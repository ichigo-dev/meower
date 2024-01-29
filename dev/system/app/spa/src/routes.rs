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

    #[to("/profile/<_..>")]
    Profile(profile::Routes),

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
                    AppRoutes::Home => view! { cx, Home },
                    AppRoutes::Profile(route) =>
                    {
                        view!
                        {
                            cx,
                            profile::Router(route=route.clone())
                        }
                    },
                    AppRoutes::NotFound => view! { cx, NotFound },
                }
            }
        )
    }
}
