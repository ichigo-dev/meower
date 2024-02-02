//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

use crate::components::{ TextField, TextFieldProps };
use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Create<G: Html>() -> View<G>
{
    let fields = create_signal(vec!
    [
        TextFieldProps
        {
            id: "account_name".to_string(),
            label: t!("pages.account.create.form.account_name.label"),
            placeholder: t!("pages.account.create.form.account_name.placeholder"),
            required: true,
            textarea: false,
        },
        TextFieldProps
        {
            id: "name".to_string(),
            label: t!("pages.account.create.form.name.label"),
            placeholder: t!("pages.account.create.form.name.placeholder"),
            required: true,
            textarea: false,
        },
    ]);

    view!
    {
        Main(heading=t!("pages.account.create.heading"))
        {
            form(class="flex flex_column flex_gap_md width_full")
            {
                Indexed
                (
                    iterable=*fields,
                    view=|field| view!
                    {
                        TextField(id=field.id, label=field.label, placeholder=field.placeholder, required=field.required, textarea=field.textarea)
                    }
                )
            }
        }
    }
}
