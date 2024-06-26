//------------------------------------------------------------------------------
//! Colors.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Colors.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colors
{
    Default,

    // Common
    Black,
    White,
    Transparent,

    // Red
    RedLighten5,
    RedLighten4,
    RedLighten3,
    RedLighten2,
    RedLighten1,
    Red,
    RedDarken1,
    RedDarken2,
    RedDarken3,
    RedDarken4,
    RedAccent1,
    RedAccent2,
    RedAccent3,
    RedAccent4,

    // Pink
    PinkLighten5,
    PinkLighten4,
    PinkLighten3,
    PinkLighten2,
    PinkLighten1,
    Pink,
    PinkDarken1,
    PinkDarken2,
    PinkDarken3,
    PinkDarken4,
    PinkAccent1,
    PinkAccent2,
    PinkAccent3,
    PinkAccent4,

    // Purple
    PurpleLighten5,
    PurpleLighten4,
    PurpleLighten3,
    PurpleLighten2,
    PurpleLighten1,
    Purple,
    PurpleDarken1,
    PurpleDarken2,
    PurpleDarken3,
    PurpleDarken4,
    PurpleAccent1,
    PurpleAccent2,
    PurpleAccent3,
    PurpleAccent4,

    // Deep Purple
    DeepPurpleLighten5,
    DeepPurpleLighten4,
    DeepPurpleLighten3,
    DeepPurpleLighten2,
    DeepPurpleLighten1,
    DeepPurple,
    DeepPurpleDarken1,
    DeepPurpleDarken2,
    DeepPurpleDarken3,
    DeepPurpleDarken4,
    DeepPurpleAccent1,
    DeepPurpleAccent2,
    DeepPurpleAccent3,
    DeepPurpleAccent4,

    // Indigo
    IndigoLighten5,
    IndigoLighten4,
    IndigoLighten3,
    IndigoLighten2,
    IndigoLighten1,
    Indigo,
    IndigoDarken1,
    IndigoDarken2,
    IndigoDarken3,
    IndigoDarken4,
    IndigoAccent1,
    IndigoAccent2,
    IndigoAccent3,
    IndigoAccent4,

    // Blue
    BlueLighten5,
    BlueLighten4,
    BlueLighten3,
    BlueLighten2,
    BlueLighten1,
    Blue,
    BlueDarken1,
    BlueDarken2,
    BlueDarken3,
    BlueDarken4,
    BlueAccent1,
    BlueAccent2,
    BlueAccent3,
    BlueAccent4,

    // Light Blue
    LightBlueLighten5,
    LightBlueLighten4,
    LightBlueLighten3,
    LightBlueLighten2,
    LightBlueLighten1,
    LightBlue,
    LightBlueDarken1,
    LightBlueDarken2,
    LightBlueDarken3,
    LightBlueDarken4,
    LightBlueAccent1,
    LightBlueAccent2,
    LightBlueAccent3,
    LightBlueAccent4,

    // Cyan
    CyanLighten5,
    CyanLighten4,
    CyanLighten3,
    CyanLighten2,
    CyanLighten1,
    Cyan,
    CyanDarken1,
    CyanDarken2,
    CyanDarken3,
    CyanDarken4,
    CyanAccent1,
    CyanAccent2,
    CyanAccent3,
    CyanAccent4,

    // Teal
    TealLighten5,
    TealLighten4,
    TealLighten3,
    TealLighten2,
    TealLighten1,
    Teal,
    TealDarken1,
    TealDarken2,
    TealDarken3,
    TealDarken4,
    TealAccent1,
    TealAccent2,
    TealAccent3,
    TealAccent4,

    // Green
    GreenLighten5,
    GreenLighten4,
    GreenLighten3,
    GreenLighten2,
    GreenLighten1,
    Green,
    GreenDarken1,
    GreenDarken2,
    GreenDarken3,
    GreenDarken4,
    GreenAccent1,
    GreenAccent2,
    GreenAccent3,
    GreenAccent4,

    // LightGreen
    LightGreenLighten5,
    LightGreenLighten4,
    LightGreenLighten3,
    LightGreenLighten2,
    LightGreenLighten1,
    LightGreen,
    LightGreenDarken1,
    LightGreenDarken2,
    LightGreenDarken3,
    LightGreenDarken4,
    LightGreenAccent1,
    LightGreenAccent2,
    LightGreenAccent3,
    LightGreenAccent4,

    // Lime
    LimeLighten5,
    LimeLighten4,
    LimeLighten3,
    LimeLighten2,
    LimeLighten1,
    Lime,
    LimeDarken1,
    LimeDarken2,
    LimeDarken3,
    LimeDarken4,
    LimeAccent1,
    LimeAccent2,
    LimeAccent3,
    LimeAccent4,

    // Yellow
    YellowLighten5,
    YellowLighten4,
    YellowLighten3,
    YellowLighten2,
    YellowLighten1,
    Yellow,
    YellowDarken1,
    YellowDarken2,
    YellowDarken3,
    YellowDarken4,
    YellowAccent1,
    YellowAccent2,
    YellowAccent3,
    YellowAccent4,

    // Amber
    AmberLighten5,
    AmberLighten4,
    AmberLighten3,
    AmberLighten2,
    AmberLighten1,
    Amber,
    AmberDarken1,
    AmberDarken2,
    AmberDarken3,
    AmberDarken4,
    AmberAccent1,
    AmberAccent2,
    AmberAccent3,
    AmberAccent4,

    // Orange
    OrangeLighten5,
    OrangeLighten4,
    OrangeLighten3,
    OrangeLighten2,
    OrangeLighten1,
    Orange,
    OrangeDarken1,
    OrangeDarken2,
    OrangeDarken3,
    OrangeDarken4,
    OrangeAccent1,
    OrangeAccent2,
    OrangeAccent3,
    OrangeAccent4,

    // Deep Orange
    DeepOrangeLighten5,
    DeepOrangeLighten4,
    DeepOrangeLighten3,
    DeepOrangeLighten2,
    DeepOrangeLighten1,
    DeepOrange,
    DeepOrangeDarken1,
    DeepOrangeDarken2,
    DeepOrangeDarken3,
    DeepOrangeDarken4,
    DeepOrangeAccent1,
    DeepOrangeAccent2,
    DeepOrangeAccent3,
    DeepOrangeAccent4,

    // Brown
    BrownLighten5,
    BrownLighten4,
    BrownLighten3,
    BrownLighten2,
    BrownLighten1,
    Brown,
    BrownDarken1,
    BrownDarken2,
    BrownDarken3,
    BrownDarken4,
    BrownAccent1,
    BrownAccent2,
    BrownAccent3,
    BrownAccent4,

    // Blue Grey
    BlueGreyLighten5,
    BlueGreyLighten4,
    BlueGreyLighten3,
    BlueGreyLighten2,
    BlueGreyLighten1,
    BlueGrey,
    BlueGreyDarken1,
    BlueGreyDarken2,
    BlueGreyDarken3,
    BlueGreyDarken4,
    BlueGreyAccent1,
    BlueGreyAccent2,
    BlueGreyAccent3,
    BlueGreyAccent4,

    // Grey
    GreyLighten5,
    GreyLighten4,
    GreyLighten3,
    GreyLighten2,
    GreyLighten1,
    Grey,
    GreyDarken1,
    GreyDarken2,
    GreyDarken3,
    GreyDarken4,
    GreyAccent1,
    GreyAccent2,
    GreyAccent3,
    GreyAccent4,

    // Theme colors
    Background,
    BackgroundText,

    Surface,
    SurfaceText,

    Filled,
    FilledLight,
    FilledDark,
    FilledText,

    Border,
    BorderLight,
    BorderDark,

    Shadow,
    ShadowLight,
    ShadowDark,

    Disabled,
    DisabledText,

    Primary,
    PrimaryLight,
    PrimaryDark,
    PrimaryText,
    Secondary,
    SecondaryLight,
    SecondaryDark,
    SecondaryText,
    Error,
    ErrorLight,
    ErrorDark,
    ErrorText,
    Warning,
    WarningLight,
    WarningDark,
    WarningText,
    Info,
    InfoLight,
    InfoDark,
    InfoText,
    Success,
    SuccessLight,
    SuccessDark,
    SuccessText,
}

impl Colors
{
    //--------------------------------------------------------------------------
    /// Gets class name.
    //--------------------------------------------------------------------------
    pub fn get_class_name( &self ) -> String
    {
        match self
        {
            Self::Default => "".to_string(),

            // Common
            Self::Black => "black".to_string(),
            Self::White => "white".to_string(),
            Self::Transparent => "transparent".to_string(),

            // Red
            Self::RedLighten5 => "red_lighten_5".to_string(),
            Self::RedLighten4 => "red_lighten_4".to_string(),
            Self::RedLighten3 => "red_lighten_3".to_string(),
            Self::RedLighten2 => "red_lighten_2".to_string(),
            Self::RedLighten1 => "red_lighten_1".to_string(),
            Self::Red => "red".to_string(),
            Self::RedDarken1 => "red_darken_1".to_string(),
            Self::RedDarken2 => "red_darken_2".to_string(),
            Self::RedDarken3 => "red_darken_3".to_string(),
            Self::RedDarken4 => "red_darken_4".to_string(),
            Self::RedAccent1 => "red_accent_1".to_string(),
            Self::RedAccent2 => "red_accent_2".to_string(),
            Self::RedAccent3 => "red_accent_3".to_string(),
            Self::RedAccent4 => "red_accent_4".to_string(),

            // Pink
            Self::PinkLighten5 => "pink_lighten_5".to_string(),
            Self::PinkLighten4 => "pink_lighten_4".to_string(),
            Self::PinkLighten3 => "pink_lighten_3".to_string(),
            Self::PinkLighten2 => "pink_lighten_2".to_string(),
            Self::PinkLighten1 => "pink_lighten_1".to_string(),
            Self::Pink => "pink".to_string(),
            Self::PinkDarken1 => "pink_darken_1".to_string(),
            Self::PinkDarken2 => "pink_darken_2".to_string(),
            Self::PinkDarken3 => "pink_darken_3".to_string(),
            Self::PinkDarken4 => "pink_darken_4".to_string(),
            Self::PinkAccent1 => "pink_accent_1".to_string(),
            Self::PinkAccent2 => "pink_accent_2".to_string(),
            Self::PinkAccent3 => "pink_accent_3".to_string(),
            Self::PinkAccent4 => "pink_accent_4".to_string(),

            // Purple
            Self::PurpleLighten5 => "purple_lighten_5".to_string(),
            Self::PurpleLighten4 => "purple_lighten_4".to_string(),
            Self::PurpleLighten3 => "purple_lighten_3".to_string(),
            Self::PurpleLighten2 => "purple_lighten_2".to_string(),
            Self::PurpleLighten1 => "purple_lighten_1".to_string(),
            Self::Purple => "purple".to_string(),
            Self::PurpleDarken1 => "purple_darken_1".to_string(),
            Self::PurpleDarken2 => "purple_darken_2".to_string(),
            Self::PurpleDarken3 => "purple_darken_3".to_string(),
            Self::PurpleDarken4 => "purple_darken_4".to_string(),
            Self::PurpleAccent1 => "purple_accent_1".to_string(),
            Self::PurpleAccent2 => "purple_accent_2".to_string(),
            Self::PurpleAccent3 => "purple_accent_3".to_string(),
            Self::PurpleAccent4 => "purple_accent_4".to_string(),

            // Deep Purple
            Self::DeepPurpleLighten5 => "deep_purple_lighten_5".to_string(),
            Self::DeepPurpleLighten4 => "deep_purple_lighten_4".to_string(),
            Self::DeepPurpleLighten3 => "deep_purple_lighten_3".to_string(),
            Self::DeepPurpleLighten2 => "deep_purple_lighten_2".to_string(),
            Self::DeepPurpleLighten1 => "deep_purple_lighten_1".to_string(),
            Self::DeepPurple => "deep_purple".to_string(),
            Self::DeepPurpleDarken1 => "deep_purple_darken_1".to_string(),
            Self::DeepPurpleDarken2 => "deep_purple_darken_2".to_string(),
            Self::DeepPurpleDarken3 => "deep_purple_darken_3".to_string(),
            Self::DeepPurpleDarken4 => "deep_purple_darken_4".to_string(),
            Self::DeepPurpleAccent1 => "deep_purple_accent_1".to_string(),
            Self::DeepPurpleAccent2 => "deep_purple_accent_2".to_string(),
            Self::DeepPurpleAccent3 => "deep_purple_accent_3".to_string(),
            Self::DeepPurpleAccent4 => "deep_purple_accent_4".to_string(),

            // Indigo
            Self::IndigoLighten5 => "indigo_lighten_5".to_string(),
            Self::IndigoLighten4 => "indigo_lighten_4".to_string(),
            Self::IndigoLighten3 => "indigo_lighten_3".to_string(),
            Self::IndigoLighten2 => "indigo_lighten_2".to_string(),
            Self::IndigoLighten1 => "indigo_lighten_1".to_string(),
            Self::Indigo => "indigo".to_string(),
            Self::IndigoDarken1 => "indigo_darken_1".to_string(),
            Self::IndigoDarken2 => "indigo_darken_2".to_string(),
            Self::IndigoDarken3 => "indigo_darken_3".to_string(),
            Self::IndigoDarken4 => "indigo_darken_4".to_string(),
            Self::IndigoAccent1 => "indigo_accent_1".to_string(),
            Self::IndigoAccent2 => "indigo_accent_2".to_string(),
            Self::IndigoAccent3 => "indigo_accent_3".to_string(),
            Self::IndigoAccent4 => "indigo_accent_4".to_string(),

            // Blue
            Self::BlueLighten5 => "blue_lighten_5".to_string(),
            Self::BlueLighten4 => "blue_lighten_4".to_string(),
            Self::BlueLighten3 => "blue_lighten_3".to_string(),
            Self::BlueLighten2 => "blue_lighten_2".to_string(),
            Self::BlueLighten1 => "blue_lighten_1".to_string(),
            Self::Blue => "blue".to_string(),
            Self::BlueDarken1 => "blue_darken_1".to_string(),
            Self::BlueDarken2 => "blue_darken_2".to_string(),
            Self::BlueDarken3 => "blue_darken_3".to_string(),
            Self::BlueDarken4 => "blue_darken_4".to_string(),
            Self::BlueAccent1 => "blue_accent_1".to_string(),
            Self::BlueAccent2 => "blue_accent_2".to_string(),
            Self::BlueAccent3 => "blue_accent_3".to_string(),
            Self::BlueAccent4 => "blue_accent_4".to_string(),

            // LightBlue
            Self::LightBlueLighten5 => "light_blue_lighten_5".to_string(),
            Self::LightBlueLighten4 => "light_blue_lighten_4".to_string(),
            Self::LightBlueLighten3 => "light_blue_lighten_3".to_string(),
            Self::LightBlueLighten2 => "light_blue_lighten_2".to_string(),
            Self::LightBlueLighten1 => "light_blue_lighten_1".to_string(),
            Self::LightBlue => "light_blue".to_string(),
            Self::LightBlueDarken1 => "light_blue_darken_1".to_string(),
            Self::LightBlueDarken2 => "light_blue_darken_2".to_string(),
            Self::LightBlueDarken3 => "light_blue_darken_3".to_string(),
            Self::LightBlueDarken4 => "light_blue_darken_4".to_string(),
            Self::LightBlueAccent1 => "light_blue_accent_1".to_string(),
            Self::LightBlueAccent2 => "light_blue_accent_2".to_string(),
            Self::LightBlueAccent3 => "light_blue_accent_3".to_string(),
            Self::LightBlueAccent4 => "light_blue_accent_4".to_string(),

            // Cyan
            Self::CyanLighten5 => "cyan_lighten_5".to_string(),
            Self::CyanLighten4 => "cyan_lighten_4".to_string(),
            Self::CyanLighten3 => "cyan_lighten_3".to_string(),
            Self::CyanLighten2 => "cyan_lighten_2".to_string(),
            Self::CyanLighten1 => "cyan_lighten_1".to_string(),
            Self::Cyan => "cyan".to_string(),
            Self::CyanDarken1 => "cyan_darken_1".to_string(),
            Self::CyanDarken2 => "cyan_darken_2".to_string(),
            Self::CyanDarken3 => "cyan_darken_3".to_string(),
            Self::CyanDarken4 => "cyan_darken_4".to_string(),
            Self::CyanAccent1 => "cyan_accent_1".to_string(),
            Self::CyanAccent2 => "cyan_accent_2".to_string(),
            Self::CyanAccent3 => "cyan_accent_3".to_string(),
            Self::CyanAccent4 => "cyan_accent_4".to_string(),

            // Teal
            Self::TealLighten5 => "teal_lighten_5".to_string(),
            Self::TealLighten4 => "teal_lighten_4".to_string(),
            Self::TealLighten3 => "teal_lighten_3".to_string(),
            Self::TealLighten2 => "teal_lighten_2".to_string(),
            Self::TealLighten1 => "teal_lighten_1".to_string(),
            Self::Teal => "teal".to_string(),
            Self::TealDarken1 => "teal_darken_1".to_string(),
            Self::TealDarken2 => "teal_darken_2".to_string(),
            Self::TealDarken3 => "teal_darken_3".to_string(),
            Self::TealDarken4 => "teal_darken_4".to_string(),
            Self::TealAccent1 => "teal_accent_1".to_string(),
            Self::TealAccent2 => "teal_accent_2".to_string(),
            Self::TealAccent3 => "teal_accent_3".to_string(),
            Self::TealAccent4 => "teal_accent_4".to_string(),

            // Green
            Self::GreenLighten5 => "green_lighten_5".to_string(),
            Self::GreenLighten4 => "green_lighten_4".to_string(),
            Self::GreenLighten3 => "green_lighten_3".to_string(),
            Self::GreenLighten2 => "green_lighten_2".to_string(),
            Self::GreenLighten1 => "green_lighten_1".to_string(),
            Self::Green => "green".to_string(),
            Self::GreenDarken1 => "green_darken_1".to_string(),
            Self::GreenDarken2 => "green_darken_2".to_string(),
            Self::GreenDarken3 => "green_darken_3".to_string(),
            Self::GreenDarken4 => "green_darken_4".to_string(),
            Self::GreenAccent1 => "green_accent_1".to_string(),
            Self::GreenAccent2 => "green_accent_2".to_string(),
            Self::GreenAccent3 => "green_accent_3".to_string(),
            Self::GreenAccent4 => "green_accent_4".to_string(),

            // LightGreen
            Self::LightGreenLighten5 => "light_green_lighten_5".to_string(),
            Self::LightGreenLighten4 => "light_green_lighten_4".to_string(),
            Self::LightGreenLighten3 => "light_green_lighten_3".to_string(),
            Self::LightGreenLighten2 => "light_green_lighten_2".to_string(),
            Self::LightGreenLighten1 => "light_green_lighten_1".to_string(),
            Self::LightGreen => "light_green".to_string(),
            Self::LightGreenDarken1 => "light_green_darken_1".to_string(),
            Self::LightGreenDarken2 => "light_green_darken_2".to_string(),
            Self::LightGreenDarken3 => "light_green_darken_3".to_string(),
            Self::LightGreenDarken4 => "light_green_darken_4".to_string(),
            Self::LightGreenAccent1 => "light_green_accent_1".to_string(),
            Self::LightGreenAccent2 => "light_green_accent_2".to_string(),
            Self::LightGreenAccent3 => "light_green_accent_3".to_string(),
            Self::LightGreenAccent4 => "light_green_accent_4".to_string(),

            // Lime
            Self::LimeLighten5 => "lime_lighten_5".to_string(),
            Self::LimeLighten4 => "lime_lighten_4".to_string(),
            Self::LimeLighten3 => "lime_lighten_3".to_string(),
            Self::LimeLighten2 => "lime_lighten_2".to_string(),
            Self::LimeLighten1 => "lime_lighten_1".to_string(),
            Self::Lime => "lime".to_string(),
            Self::LimeDarken1 => "lime_darken_1".to_string(),
            Self::LimeDarken2 => "lime_darken_2".to_string(),
            Self::LimeDarken3 => "lime_darken_3".to_string(),
            Self::LimeDarken4 => "lime_darken_4".to_string(),
            Self::LimeAccent1 => "lime_accent_1".to_string(),
            Self::LimeAccent2 => "lime_accent_2".to_string(),
            Self::LimeAccent3 => "lime_accent_3".to_string(),
            Self::LimeAccent4 => "lime_accent_4".to_string(),

            // Yellow
            Self::YellowLighten5 => "yellow_lighten_5".to_string(),
            Self::YellowLighten4 => "yellow_lighten_4".to_string(),
            Self::YellowLighten3 => "yellow_lighten_3".to_string(),
            Self::YellowLighten2 => "yellow_lighten_2".to_string(),
            Self::YellowLighten1 => "yellow_lighten_1".to_string(),
            Self::Yellow => "yellow".to_string(),
            Self::YellowDarken1 => "yellow_darken_1".to_string(),
            Self::YellowDarken2 => "yellow_darken_2".to_string(),
            Self::YellowDarken3 => "yellow_darken_3".to_string(),
            Self::YellowDarken4 => "yellow_darken_4".to_string(),
            Self::YellowAccent1 => "yellow_accent_1".to_string(),
            Self::YellowAccent2 => "yellow_accent_2".to_string(),
            Self::YellowAccent3 => "yellow_accent_3".to_string(),
            Self::YellowAccent4 => "yellow_accent_4".to_string(),

            // Amber
            Self::AmberLighten5 => "amber_lighten_5".to_string(),
            Self::AmberLighten4 => "amber_lighten_4".to_string(),
            Self::AmberLighten3 => "amber_lighten_3".to_string(),
            Self::AmberLighten2 => "amber_lighten_2".to_string(),
            Self::AmberLighten1 => "amber_lighten_1".to_string(),
            Self::Amber => "amber".to_string(),
            Self::AmberDarken1 => "amber_darken_1".to_string(),
            Self::AmberDarken2 => "amber_darken_2".to_string(),
            Self::AmberDarken3 => "amber_darken_3".to_string(),
            Self::AmberDarken4 => "amber_darken_4".to_string(),
            Self::AmberAccent1 => "amber_accent_1".to_string(),
            Self::AmberAccent2 => "amber_accent_2".to_string(),
            Self::AmberAccent3 => "amber_accent_3".to_string(),
            Self::AmberAccent4 => "amber_accent_4".to_string(),

            // Orange
            Self::OrangeLighten5 => "orange_lighten_5".to_string(),
            Self::OrangeLighten4 => "orange_lighten_4".to_string(),
            Self::OrangeLighten3 => "orange_lighten_3".to_string(),
            Self::OrangeLighten2 => "orange_lighten_2".to_string(),
            Self::OrangeLighten1 => "orange_lighten_1".to_string(),
            Self::Orange => "orange".to_string(),
            Self::OrangeDarken1 => "orange_darken_1".to_string(),
            Self::OrangeDarken2 => "orange_darken_2".to_string(),
            Self::OrangeDarken3 => "orange_darken_3".to_string(),
            Self::OrangeDarken4 => "orange_darken_4".to_string(),
            Self::OrangeAccent1 => "orange_accent_1".to_string(),
            Self::OrangeAccent2 => "orange_accent_2".to_string(),
            Self::OrangeAccent3 => "orange_accent_3".to_string(),
            Self::OrangeAccent4 => "orange_accent_4".to_string(),

            // Deep Orange
            Self::DeepOrangeLighten5 => "deep_orange_lighten_5".to_string(),
            Self::DeepOrangeLighten4 => "deep_orange_lighten_4".to_string(),
            Self::DeepOrangeLighten3 => "deep_orange_lighten_3".to_string(),
            Self::DeepOrangeLighten2 => "deep_orange_lighten_2".to_string(),
            Self::DeepOrangeLighten1 => "deep_orange_lighten_1".to_string(),
            Self::DeepOrange => "deep_orange".to_string(),
            Self::DeepOrangeDarken1 => "deep_orange_darken_1".to_string(),
            Self::DeepOrangeDarken2 => "deep_orange_darken_2".to_string(),
            Self::DeepOrangeDarken3 => "deep_orange_darken_3".to_string(),
            Self::DeepOrangeDarken4 => "deep_orange_darken_4".to_string(),
            Self::DeepOrangeAccent1 => "deep_orange_accent_1".to_string(),
            Self::DeepOrangeAccent2 => "deep_orange_accent_2".to_string(),
            Self::DeepOrangeAccent3 => "deep_orange_accent_3".to_string(),
            Self::DeepOrangeAccent4 => "deep_orange_accent_4".to_string(),

            // Brown
            Self::BrownLighten5 => "brown_lighten_5".to_string(),
            Self::BrownLighten4 => "brown_lighten_4".to_string(),
            Self::BrownLighten3 => "brown_lighten_3".to_string(),
            Self::BrownLighten2 => "brown_lighten_2".to_string(),
            Self::BrownLighten1 => "brown_lighten_1".to_string(),
            Self::Brown => "brown".to_string(),
            Self::BrownDarken1 => "brown_darken_1".to_string(),
            Self::BrownDarken2 => "brown_darken_2".to_string(),
            Self::BrownDarken3 => "brown_darken_3".to_string(),
            Self::BrownDarken4 => "brown_darken_4".to_string(),
            Self::BrownAccent1 => "brown_accent_1".to_string(),
            Self::BrownAccent2 => "brown_accent_2".to_string(),
            Self::BrownAccent3 => "brown_accent_3".to_string(),
            Self::BrownAccent4 => "brown_accent_4".to_string(),

            // Blue Grey
            Self::BlueGreyLighten5 => "blue_grey_lighten_5".to_string(),
            Self::BlueGreyLighten4 => "blue_grey_lighten_4".to_string(),
            Self::BlueGreyLighten3 => "blue_grey_lighten_3".to_string(),
            Self::BlueGreyLighten2 => "blue_grey_lighten_2".to_string(),
            Self::BlueGreyLighten1 => "blue_grey_lighten_1".to_string(),
            Self::BlueGrey => "blue_grey".to_string(),
            Self::BlueGreyDarken1 => "blue_grey_darken_1".to_string(),
            Self::BlueGreyDarken2 => "blue_grey_darken_2".to_string(),
            Self::BlueGreyDarken3 => "blue_grey_darken_3".to_string(),
            Self::BlueGreyDarken4 => "blue_grey_darken_4".to_string(),
            Self::BlueGreyAccent1 => "blue_grey_accent_1".to_string(),
            Self::BlueGreyAccent2 => "blue_grey_accent_2".to_string(),
            Self::BlueGreyAccent3 => "blue_grey_accent_3".to_string(),
            Self::BlueGreyAccent4 => "blue_grey_accent_4".to_string(),

            // Grey
            Self::GreyLighten5 => "grey_lighten_5".to_string(),
            Self::GreyLighten4 => "grey_lighten_4".to_string(),
            Self::GreyLighten3 => "grey_lighten_3".to_string(),
            Self::GreyLighten2 => "grey_lighten_2".to_string(),
            Self::GreyLighten1 => "grey_lighten_1".to_string(),
            Self::Grey => "grey".to_string(),
            Self::GreyDarken1 => "grey_darken_1".to_string(),
            Self::GreyDarken2 => "grey_darken_2".to_string(),
            Self::GreyDarken3 => "grey_darken_3".to_string(),
            Self::GreyDarken4 => "grey_darken_4".to_string(),
            Self::GreyAccent1 => "grey_accent_1".to_string(),
            Self::GreyAccent2 => "grey_accent_2".to_string(),
            Self::GreyAccent3 => "grey_accent_3".to_string(),
            Self::GreyAccent4 => "grey_accent_4".to_string(),

            // Theme colors
            Self::Background => "background".to_string(),
            Self::BackgroundText => "background_text".to_string(),

            Self::Surface => "surface".to_string(),
            Self::SurfaceText => "surface_text".to_string(),

            Self::Filled => "filled".to_string(),
            Self::FilledLight => "filled_light".to_string(),
            Self::FilledDark => "filled_dark".to_string(),
            Self::FilledText => "filled_text".to_string(),

            Self::Border => "border".to_string(),
            Self::BorderLight => "border_light".to_string(),
            Self::BorderDark => "border_dark".to_string(),

            Self::Shadow => "shadow".to_string(),
            Self::ShadowLight => "shadow_light".to_string(),
            Self::ShadowDark => "shadow_dark".to_string(),

            Self::Disabled => "disabled".to_string(),
            Self::DisabledText => "disabled_text".to_string(),

            Self::Primary => "primary".to_string(),
            Self::PrimaryLight => "primary_light".to_string(),
            Self::PrimaryDark => "primary_dark".to_string(),
            Self::PrimaryText => "primary_text".to_string(),
            Self::Secondary => "secondary".to_string(),
            Self::SecondaryLight => "secondary_light".to_string(),
            Self::SecondaryDark => "secondary_dark".to_string(),
            Self::SecondaryText => "secondary_text".to_string(),
            Self::Error => "error".to_string(),
            Self::ErrorLight => "error_light".to_string(),
            Self::ErrorDark => "error_dark".to_string(),
            Self::ErrorText => "error_text".to_string(),
            Self::Warning => "warning".to_string(),
            Self::WarningLight => "warning_light".to_string(),
            Self::WarningDark => "warning_dark".to_string(),
            Self::WarningText => "warning_text".to_string(),
            Self::Info => "info".to_string(),
            Self::InfoLight => "info_light".to_string(),
            Self::InfoDark => "info_dark".to_string(),
            Self::InfoText => "info_text".to_string(),
            Self::Success => "success".to_string(),
            Self::SuccessLight => "success_light".to_string(),
            Self::SuccessDark => "success_dark".to_string(),
            Self::SuccessText => "success_text".to_string(),
        }
    }
}

impl Default for Colors
{
    fn default() -> Self
    {
        Self::Default
    }
}

impl Into<ReadSignal<Colors>> for Colors
{
    fn into( self ) -> ReadSignal<Self>
    {
        *create_signal(self)
    }
}
