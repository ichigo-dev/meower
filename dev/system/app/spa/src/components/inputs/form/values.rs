//------------------------------------------------------------------------------
//! FormValues.
//------------------------------------------------------------------------------

use std::collections::HashMap;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// FormValues.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct FormValues(HashMap<String, String>);

impl Default for FormValues
{
    fn default() -> Self
    {
        Self(HashMap::new())
    }
}

impl FormValues
{
    //--------------------------------------------------------------------------
    /// Creates a new instance.
    //--------------------------------------------------------------------------
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        Self(HashMap::new())
    }

    //--------------------------------------------------------------------------
    /// Gets the value of a field.
    //--------------------------------------------------------------------------
    #[allow(dead_code)]
    pub fn get( &self, name: &str ) -> Option<&String>
    {
        self.0.get(name)
    }

    //--------------------------------------------------------------------------
    /// Sets the value of a field.
    //--------------------------------------------------------------------------
    #[allow(dead_code)]
    pub fn set( &mut self, name: &str, value: &str )
    {
        self.0.insert(name.to_string(), value.to_string());
    }

    //--------------------------------------------------------------------------
    /// Removes a field.
    //--------------------------------------------------------------------------
    pub fn remove( &mut self, name: &str )
    {
        self.0.remove(name);
    }
}

impl Into<ReadSignal<FormValues>> for FormValues
{
    fn into(self) -> ReadSignal<FormValues>
    {
        *create_signal(self)
    }
}

impl Into<Signal<FormValues>> for FormValues
{
    fn into(self) -> Signal<FormValues>
    {
        create_signal(self)
    }
}
