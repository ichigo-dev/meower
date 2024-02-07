//------------------------------------------------------------------------------
//! Pagination.
//------------------------------------------------------------------------------

mod props;
mod variant;

pub use props::PaginationProps;
pub use variant::*;

use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[component]
pub fn Pagination<G: Html>( props: PaginationProps<G> ) -> View<G>
{
    let total = move || { props.count.get() / props.per_page.get() };
    let classes = move ||
    {
        let mut classes = vec!
        [
            "ui_pagination".to_string(),
            props.classes.get_clone(),
            props.variant.get_clone().get_class_name(),
        ];
        if total() <= 1 { classes.push("hide".to_string()) }
        classes.retain(|c| !c.is_empty());
        classes.join(" ")
    };

    let pages = create_signal((1..=total()).collect::<Vec<usize>>());
    view!
    {
        div(class=classes(), ..props.attributes)
        {
            (
                if props.show_first_button.get()
                {
                    view!
                    {
                        button
                        (
                            class="page_first",
                            disabled=props.page.get() == 1,
                            on:click=move |_|
                            {
                                props.page.set(1)
                            }
                        )
                    }
                }
                else { view! {} }
            )
            (
                if props.show_prev_button.get()
                {
                    view!
                    {
                        button
                        (
                            class="page_prev",
                            disabled=props.page.get() == 1,
                            on:click=move |_|
                            {
                                props.page.set(props.page.get() - 1)
                            }
                        )
                    }
                }
                else { view! {} }
            )
            Indexed
            (
                iterable=*pages,
                view=move |i|
                {
                    let classes = move ||
                    {
                        if props.page.get() == i
                        {
                            "page_no page_now"
                        }
                        else
                        {
                            "page_no"
                        }
                    };
                    view!
                    {
                        button
                        (
                            class=classes(),
                            disabled=props.page.get() == i,
                            on:click=move |_|
                            {
                                props.page.set(i)
                            }
                        )
                        {
                            (i)
                        }
                    }
                }
            )
            (
                if props.show_next_button.get()
                {
                    view!
                    {
                        button
                        (
                            class="page_next",
                            disabled=props.page.get() == total(),
                            on:click=move |_|
                            {
                                props.page.set(props.page.get() + 1)
                            }
                        )
                    }
                }
                else { view! {} }
            )
            (
                if props.show_last_button.get()
                {
                    view!
                    {
                        button
                        (
                            class="page_last",
                            disabled=props.page.get() == total(),
                            on:click=move |_|
                            {
                                props.page.set(total())
                            }
                        )
                    }
                }
                else { view! {} }
            )
        }
    }
}
