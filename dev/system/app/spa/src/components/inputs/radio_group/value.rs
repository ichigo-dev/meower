//------------------------------------------------------------------------------
//! RadioValue.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// RadioValue.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct RadioValue(pub String);

impl RadioValue
{
    //--------------------------------------------------------------------------
    /// Creates a new instance.
    //--------------------------------------------------------------------------
    pub fn new( value: &str ) -> Self
    {
        Self(value.to_string())
    }

    //--------------------------------------------------------------------------
    /// Gets the value.
    //--------------------------------------------------------------------------
    pub fn get( &self ) -> String
    {
        self.0.clone()
    }
}

impl Default for RadioValue
{
    fn default() -> Self
    {
        Self("".to_string())
    }
}

impl Into<ReadSignal<RadioValue>> for RadioValue
{
    fn into(self) -> ReadSignal<RadioValue>
    {
        *create_signal(self)
    }
}

impl Into<Signal<RadioValue>> for RadioValue
{
    fn into(self) -> Signal<RadioValue>
    {
        create_signal(self)
    }
}
