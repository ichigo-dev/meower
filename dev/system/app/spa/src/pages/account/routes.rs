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
    // Account
    #[to("/")]
    Detail,

    #[to("/create")]
    Create,

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
            Routes::Detail => view! { Detail },
            Routes::Create => view! { Create },
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
            Routes::NotFound => view! { NotFound },
        })
    }
}
