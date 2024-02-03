//------------------------------------------------------------------------------
//! Props.
//------------------------------------------------------------------------------

use super::animation::SkeletonAnimation;
use super::shape::SkeletonShape;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Props)]
pub struct SkeletonProps
{
    #[prop(default)]
    pub animation: ReadSignal<SkeletonAnimation>,

    #[prop(default)]
    pub classes: ReadSignal<String>,

    #[prop(default)]
    pub shape: ReadSignal<SkeletonShape>,
}
