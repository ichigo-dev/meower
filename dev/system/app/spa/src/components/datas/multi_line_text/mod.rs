//------------------------------------------------------------------------------
//! MultiLineText.
//------------------------------------------------------------------------------

mod props;

pub use props::MultiLineTextProps;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn MultiLineText<G: Html>( props: MultiLineTextProps<G> ) -> View<G>
{
    let lines = create_signal(Vec::new());
    create_effect(move || 
    {
        let text = props.text.get_clone();
        let text_lines = text
            .split("\n")
            .map(|line| line.to_string())
            .collect();
        lines.set(text_lines);
    });

    view!
    {
        p
        (
            class=props.classes,
            ref=props.node_ref,
            ..props.attributes
        )
        {
            Indexed
            (
                iterable=*lines,
                view=move |line|
                {
                    let cloned_line = line.clone();
                    view!
                    {
                        (cloned_line)
                        (
                            if &line != lines.get_clone().last().unwrap()
                            {
                                view! { br() }
                            }
                            else
                            {
                                view! {}
                            }
                        )
                    }
                },
            )
        }
    }
}
