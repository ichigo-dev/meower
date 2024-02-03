//------------------------------------------------------------------------------
//! Thickness.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Thickness.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProgressThickness
{
    Thin,
    Normal,
    Thick,
}

impl ProgressThickness
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Thin => "thin".to_string(),
            Self::Normal => "".to_string(),
            Self::Thick => "thick".to_string(),
        }
    }
}

impl Default for ProgressThickness
{
    fn default() -> Self
    {
        Self::Normal
    }
}

impl Into<ReadSignal<ProgressThickness>> for ProgressThickness
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
