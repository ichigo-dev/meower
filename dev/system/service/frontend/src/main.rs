//------------------------------------------------------------------------------
//! Frontend for the Meower application.
//------------------------------------------------------------------------------

mod routes;
mod pages;
mod layouts;
mod features;

use routes::AppRouter;
use layouts::Layout;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Application root.
//------------------------------------------------------------------------------
#[component]
pub fn Root<G: Html>( cx: Scope ) -> View<G>
{
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
