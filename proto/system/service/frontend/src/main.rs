//------------------------------------------------------------------------------
//! Frontend for the Meower application.
//------------------------------------------------------------------------------

mod api;
mod routes;
mod pages;
mod layouts;
mod features;

use routes::AppRouter;
use layouts::Layout;

use sycamore::prelude::*;
use reqwest::Client;


//------------------------------------------------------------------------------
/// Application state.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    client: Client,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub(crate) fn new( client: Client ) -> Self
    {
        Self { client }
    }

    //--------------------------------------------------------------------------
    /// Returns the client.
    //--------------------------------------------------------------------------
    pub(crate) fn client( &self ) -> &Client
    {
        &self.client
    }
}

//------------------------------------------------------------------------------
/// Application root.
//------------------------------------------------------------------------------
#[component]
pub fn Root<G: Html>( cx: Scope ) -> View<G>
{
    let client = Client::new();
    let app_state = AppState::new(client);
    provide_context(cx, app_state);
    view!
    {
        cx,
        Layout(child=view!{ cx, AppRouter })
    }
}


//------------------------------------------------------------------------------
/// Application entry point.
//------------------------------------------------------------------------------
fn main()
{
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(Root);
}
