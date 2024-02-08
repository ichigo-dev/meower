//------------------------------------------------------------------------------
//! Typography.
//------------------------------------------------------------------------------

mod font_size;
mod font_weight;
mod props;

pub use font_size::TypographyFontSize;
pub use font_weight::TypographyFontWeight;
pub use props::TypographyProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Typography<G: Html>( props: TypographyProps<G> ) -> View<G>
{
    let classes = move ||
    {
        let mut classes = vec!
        [
            props.classes.get_clone(),
            props.font_size.get().get_class_name(),
            props.font_weight.get().get_class_name(),
        ];
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let children = props.children.call();
    view!
    {
        p(class=classes())
        {
            (children)
        }
    }
}
