//------------------------------------------------------------------------------
//! Kind.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Kind.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconKind
{
    Default,

    AngleDown,
    AngleLeft,
    AngleRight,
    AngleUp,
    AnglesDown,
    AnglesLeft,
    AnglesRight,
    AnglesUp,

    CaretDown,
    CaretLeft,
    CaretRight,
    CaretUp,
    LeftRight,

    Ellipsis,
    Plus,
    Minus,
    Check,
    Copy,
    Pen,
    Xmark,

    Desktop,
    Laptop,
    Tablet,
    Mobile,

    Building,
    Envelope,
    Birthday,
    Gender,
    Location,
    Phone,
}

impl IconKind
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),

            Self::AngleDown => "icon_angle_down".to_string(),
            Self::AngleLeft => "icon_angle_left".to_string(),
            Self::AngleRight => "icon_angle_right".to_string(),
            Self::AngleUp => "icon_angle_up".to_string(),
            Self::AnglesDown => "icon_angles_down".to_string(),
            Self::AnglesLeft => "icon_angles_left".to_string(),
            Self::AnglesRight => "icon_angles_right".to_string(),
            Self::AnglesUp => "icon_angles_up".to_string(),

            Self::CaretDown => "icon_caret_down".to_string(),
            Self::CaretLeft => "icon_caret_left".to_string(),
            Self::CaretRight => "icon_caret_right".to_string(),
            Self::CaretUp => "icon_caret_up".to_string(),
            Self::LeftRight => "icon_left_right".to_string(),

            Self::Ellipsis => "icon_ellipsis".to_string(),
            Self::Plus => "icon_plus".to_string(),
            Self::Minus => "icon_minus".to_string(),
            Self::Check => "icon_check".to_string(),
            Self::Copy => "icon_copy".to_string(),
            Self::Pen => "icon_pen".to_string(),
            Self::Xmark => "icon_xmark".to_string(),

            Self::Desktop => "icon_desktop".to_string(),
            Self::Laptop => "icon_laptop".to_string(),
            Self::Tablet => "icon_tablet".to_string(),
            Self::Mobile => "icon_mobile".to_string(),

            Self::Building => "icon_building".to_string(),
            Self::Envelope => "icon_envelope".to_string(),
            Self::Birthday => "icon_birthday".to_string(),
            Self::Gender => "icon_gender".to_string(),
            Self::Location => "icon_location".to_string(),
            Self::Phone => "icon_phone".to_string(),
        }
    }
}

impl Default for IconKind
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<IconKind>> for IconKind
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
