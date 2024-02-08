//------------------------------------------------------------------------------
//! FontWeight.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// FontWeight.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypographyFontWeight
{
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl TypographyFontWeight
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Thin => "text_thin".to_string(),
            Self::ExtraLight => "text_extra_light".to_string(),
            Self::Light => "text_light".to_string(),
            Self::Regular => "text_regular".to_string(),
            Self::Medium => "text_medium".to_string(),
            Self::SemiBold => "text_semi_bold".to_string(),
            Self::Bold => "text_bold".to_string(),
            Self::ExtraBold => "text_extra_bold".to_string(),
            Self::Black => "text_black".to_string(),
        }
    }
}

impl Default for TypographyFontWeight
{
    fn default() -> Self
    {
        Self::Medium
    }
}

impl Into<ReadSignal<TypographyFontWeight>> for TypographyFontWeight
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
