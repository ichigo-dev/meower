//------------------------------------------------------------------------------
//! Common Props.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// StringProp.
//------------------------------------------------------------------------------
pub struct StringProp(pub &'static str);

impl Into<ReadSignal<String>> for StringProp
{
    fn into( self ) -> ReadSignal<String>
    {
        *create_signal(self.0.to_string())
    }
}


//------------------------------------------------------------------------------
/// BoolProp.
//------------------------------------------------------------------------------
pub struct BoolProp(pub bool);

impl Into<ReadSignal<bool>> for BoolProp
{
    fn into( self ) -> ReadSignal<bool>
    {
        *create_signal(self.0)
    }
}


//------------------------------------------------------------------------------
/// UsizeProp.
//------------------------------------------------------------------------------
pub struct UsizeProp(pub usize);

impl Into<ReadSignal<usize>> for UsizeProp
{
    fn into( self ) -> ReadSignal<usize>
    {
        *create_signal(self.0)
    }
}
