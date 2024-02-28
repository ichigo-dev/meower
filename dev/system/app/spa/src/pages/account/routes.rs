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
    #[to("/profile/create")]
    ProfileCreate,

    #[to("/profile/edit/<token>")]
    ProfileEdit
    {
        token: String,
    },

    #[to("/create")]
    Create,

    #[to("/<account_name>")]
    Detail
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
            Routes::ProfileCreate => view! { profile::Create },
            Routes::ProfileEdit { token } =>
            {
                view! { profile::Edit(token=token.clone()) }
            },
            Routes::Create => view! { Create },
            Routes::Detail { account_name } =>
            {
                view! { Detail(account_name=account_name.clone()) }
            },
            Routes::NotFound => view! { NotFound },
        })
    }
}
