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
    // Profile
    #[to("/profile/create")]
    ProfileCreate,

    #[to("/profile/edit/<token>")]
    ProfileEdit
    {
        token: String,
    },

    // Group
    #[to("/group/create")]
    GroupCreate,

    #[to("/group/edit/<group_name>")]
    GroupEdit
    {
        group_name: String,
    },

    // Account
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
            Routes::GroupCreate => view! { group::Create },
            Routes::GroupEdit { group_name } =>
            {
                view! { group::Edit(group_name=group_name.clone()) }
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
