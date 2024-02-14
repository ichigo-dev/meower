//------------------------------------------------------------------------------
//! Size.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Size.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize
{
    Full,
    FullViewport,
    Half,
    HalfViewport,
    Zero,
    XXXXXS,
    XXXXS,
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
    XXXXXXL,
    XXXXXXXL,
    XXXXXXXXL,
    XXXXXXXXXL,
    XXXXXXXXXXL,
    BreakpointXS,
    BreakpointSM,
    BreakpointMD,
    BreakpointLG,
    BreakpointXL,
    Avatar,
}

impl AvatarSize
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Full => "size_full".to_string(),
            Self::FullViewport => "size_full_viewport".to_string(),
            Self::Half => "size_half".to_string(),
            Self::HalfViewport => "size_half_viewport".to_string(),
            Self::Zero => "size_zero".to_string(),
            Self::XXXXXS => "size_5xs".to_string(),
            Self::XXXXS => "size_4xs".to_string(),
            Self::XXXS => "size_3xs".to_string(),
            Self::XXS => "size_2xs".to_string(),
            Self::XS => "size_xs".to_string(),
            Self::SM => "size_sm".to_string(),
            Self::MD => "size_md".to_string(),
            Self::LG => "size_lg".to_string(),
            Self::XL => "size_xl".to_string(),
            Self::XXL => "size_2xl".to_string(),
            Self::XXXL => "size_3xl".to_string(),
            Self::XXXXL => "size_4xl".to_string(),
            Self::XXXXXL => "size_5xl".to_string(),
            Self::XXXXXXL => "size_6xl".to_string(),
            Self::XXXXXXXL => "size_7xl".to_string(),
            Self::XXXXXXXXL => "size_8xl".to_string(),
            Self::XXXXXXXXXL => "size_9xl".to_string(),
            Self::XXXXXXXXXXL => "size_10xl".to_string(),
            Self::BreakpointXS => "size_breakpoint_xs".to_string(),
            Self::BreakpointSM => "size_breakpoint_sm".to_string(),
            Self::BreakpointMD => "size_breakpoint_md".to_string(),
            Self::BreakpointLG => "size_breakpoint_lg".to_string(),
            Self::BreakpointXL => "size_breakpoint_xl".to_string(),
            Self::Avatar => "".to_string(),
        }
    }
}

impl Default for AvatarSize
{
    fn default() -> Self
    {
        Self::Avatar
    }
}

impl Into<ReadSignal<AvatarSize>> for AvatarSize
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
