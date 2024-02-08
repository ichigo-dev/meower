//------------------------------------------------------------------------------
//! FontSize.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// FontSize.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypographyFontSize
{
    XXXS,
    XXS,
    XS,
    SM,
    MD,
    LG,
    XL,
    XXL,
    XXXL,
    XXXXL,
    XXXXXL,
}

impl TypographyFontSize
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::XXXS => "fs_3xs".to_string(),
            Self::XXS => "fs_2xs".to_string(),
            Self::XS => "fs_xs".to_string(),
            Self::SM => "fs_sm".to_string(),
            Self::MD => "fs_md".to_string(),
            Self::LG => "fs_lg".to_string(),
            Self::XL => "fs_xl".to_string(),
            Self::XXL => "fs_2xl".to_string(),
            Self::XXXL => "fs_3xl".to_string(),
            Self::XXXXL => "fs_4xl".to_string(),
            Self::XXXXXL => "fs_5xl".to_string(),
        }
    }
}

impl Default for TypographyFontSize
{
    fn default() -> Self
    {
        Self::MD
    }
}

impl Into<ReadSignal<TypographyFontSize>> for TypographyFontSize
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
