//------------------------------------------------------------------------------
//! Application routes.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::pages::*;

use sycamore::prelude::*;
use sycamore_router::{ Route, Router, HistoryIntegration };


//------------------------------------------------------------------------------
/// Application routes.
//------------------------------------------------------------------------------
#[derive(Clone, Route)]
enum AppRoutes
{
    #[to("/")]
    Home,

    #[to("/dev/<_..>")]
    Dev(dev::Routes),

    #[to("/account/<_..>")]
    Account(account::Routes),

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Application router.
//------------------------------------------------------------------------------
#[component]
pub fn AppRouter<G: Html>() -> View<G>
{
    view!
    {
        Router
        (
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>|
            {
                match route.get_clone()
                {
                    AppRoutes::Home => view! { Home },
                    AppRoutes::Dev(route) =>
                    {
                        let state: AppState = use_context();
                        if state.config.dev_mode
                        {
                            return view!{ dev::Router(route=route.clone()) };
                        }
                        view! { NotFound }
                    },
                    AppRoutes::Account(route) =>
                    {
                        view! { account::Router(route=route.clone()) }
                    },
                    AppRoutes::NotFound => view! { NotFound },
                }
            }
        )
    }
}
