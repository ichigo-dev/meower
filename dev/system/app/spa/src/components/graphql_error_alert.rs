//------------------------------------------------------------------------------
//! GraphQL error alert component.
//------------------------------------------------------------------------------

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// GraphQLErrorAlert
//------------------------------------------------------------------------------
#[component]
pub fn GraphQLErrorAlert<G: Html>() -> View<G>
{
    view!
    {
        div(class="ui_alert outlined error")
        {
            (t!("common.api.graphql.error"))
        }
    }
}
