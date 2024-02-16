//------------------------------------------------------------------------------
//! Tag.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Tag.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoxTag
{
    Div,
    Section,
    Article,
    Aside,
    Header,
    Footer,
    Main,
    Nav,
    Span,
    Button,
}

impl Default for BoxTag
{
    fn default() -> Self
    {
        Self::Div
    }
}

impl Into<ReadSignal<BoxTag>> for BoxTag
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
