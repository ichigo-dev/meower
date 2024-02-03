//------------------------------------------------------------------------------
//! SkeletonExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn SkeletonExamples<G: Html>() -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.skeleton.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.skeleton.basic.heading"))
            }
            div(class="flex flex_column flex_gap_md flex_align_start width_full")
            {
                Skeleton(shape=SkeletonShape::Text.into())
                Skeleton(shape=SkeletonShape::Circle.into())
                Skeleton(shape=SkeletonShape::Box.into())
                Skeleton(shape=SkeletonShape::Box.into())
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.skeleton.pulse.heading"))
            }
            div(class="flex flex_column flex_gap_md flex_align_start width_full")
            {
                Skeleton
                (
                    animation=SkeletonAnimation::Pulse.into(),
                    shape=SkeletonShape::Text.into(),
                )
                Skeleton
                (
                    animation=SkeletonAnimation::Pulse.into(),
                    shape=SkeletonShape::Circle.into(),
                )
                Skeleton
                (
                    animation=SkeletonAnimation::Pulse.into(),
                    shape=SkeletonShape::Box.into(),
                )
                Skeleton
                (
                    animation=SkeletonAnimation::Pulse.into(),
                    shape=SkeletonShape::Box.into(),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.skeleton.wave.heading"))
            }
            div(class="flex flex_column flex_gap_md flex_align_start width_full")
            {
                Skeleton
                (
                    animation=SkeletonAnimation::Wave.into(),
                    shape=SkeletonShape::Text.into(),
                )
                Skeleton
                (
                    animation=SkeletonAnimation::Wave.into(),
                    shape=SkeletonShape::Circle.into(),
                )
                Skeleton
                (
                    animation=SkeletonAnimation::Wave.into(),
                    shape=SkeletonShape::Box.into(),
                )
                Skeleton
                (
                    animation=SkeletonAnimation::Wave.into(),
                    shape=SkeletonShape::Box.into(),
                )
            }
        }
    }
}
