//------------------------------------------------------------------------------
//! Provider.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Context.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub struct SnackbarContext<G: Html>
{
    pub auto_hide_timer_id: Signal<i32>,
    pub open_node_ref: Signal<Option<NodeRef<G>>>,
}


//------------------------------------------------------------------------------
/// Provider.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component(inline_props)]
pub fn SnackbarProvider<G: Html>( children: Children<G> ) -> View<G>
{
    let context: SnackbarContext<G> = SnackbarContext
    {
        auto_hide_timer_id: create_signal(0),
        open_node_ref: create_signal(None),
    };
    provide_context(context);

    let children = children.call();
    view! { (children) }
}
