//------------------------------------------------------------------------------
//! Account routes.
//------------------------------------------------------------------------------

use super::*;
use crate::pages::notfound::NotFound;

use sycamore::prelude::*;
use sycamore_router::Route;


//------------------------------------------------------------------------------
/// Routes.
//------------------------------------------------------------------------------
#[derive(Clone, Route)]
pub enum Routes
{
    #[to("/create")]
    Create,

    #[to("/<account_name>")]
    Profile
    {
        account_name: String,
    },

    #[not_found]
    NotFound,
}


//------------------------------------------------------------------------------
/// Router.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Router<G: Html>( route: Routes ) -> View<G>
{
    view!
    {
        (match &route
        {
            Routes::Create => view! { Create },
            Routes::Profile { account_name } =>
            {
                view! { Profile(account_name=account_name.clone()) }
            },
            Routes::NotFound => view! { NotFound },
        })
    }
}
