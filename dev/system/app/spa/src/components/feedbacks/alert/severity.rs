//------------------------------------------------------------------------------
//! Severity.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Severity.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity
{
    Error,
    Warning,
    Info,
    Success,
}

impl AlertSeverity
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Error => "error".to_string(),
            Self::Warning => "warning".to_string(),
            Self::Info => "info".to_string(),
            Self::Success => "success".to_string(),
        }
    }
}

impl Default for AlertSeverity
{
    fn default() -> Self
    {
        Self::Error
    }
}

impl Into<ReadSignal<AlertSeverity>> for AlertSeverity
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
