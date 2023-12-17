//------------------------------------------------------------------------------
//! Frontend for the Meower application.
//------------------------------------------------------------------------------

mod api;
mod routes;
mod pages;
mod layouts;
mod features;

use meower_core::*;
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
    config: Config,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub(crate) fn new( client: Client, config: Config ) -> Self
    {
        Self { client, config }
    }

    //--------------------------------------------------------------------------
    /// Returns the client.
    //--------------------------------------------------------------------------
    pub(crate) fn client( &self ) -> &Client
    {
        &self.client
    }

    //--------------------------------------------------------------------------
    /// Returns the config.
    //--------------------------------------------------------------------------
    pub(crate) fn config( &self ) -> &Config
    {
        &self.config
    }
}

//------------------------------------------------------------------------------
/// Application root.
//------------------------------------------------------------------------------
#[component]
pub fn Root<G: Html>( cx: Scope ) -> View<G>
{
    let client = Client::new();
    let config = Config::new();
    let app_state = AppState::new(client, config);
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
    sycamore::render(Root);
}
