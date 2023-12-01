//------------------------------------------------------------------------------
//! Frontend for the Meower application.
//------------------------------------------------------------------------------

use sycamore::prelude::*;

fn main()
{
    sycamore::render(|cx|
    {
        view!
        {
            cx,
            p { "Hello Meower!" }
        }
    });
}
