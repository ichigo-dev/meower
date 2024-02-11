//------------------------------------------------------------------------------
//! Direction.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Direction.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LabelDirection
{
    Row,
    Column,
}

impl LabelDirection
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Row => "flex_row".to_string(),
            Self::Column => "flex_column".to_string(),
        }
    }
}

impl Default for LabelDirection
{
    fn default() -> Self
    {
        Self::Column
    }
}

impl Into<ReadSignal<LabelDirection>> for LabelDirection
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
