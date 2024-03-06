//------------------------------------------------------------------------------
//! Account detail page.
//------------------------------------------------------------------------------

mod account_profiles;
mod account_profile_card;

use account_profiles::AccountProfiles;

use crate::AppState;
use crate::components::*;
use crate::layouts::application::{ Layout, Main };
use crate::utils::props::*;
use crate::variables::*;

use rust_i18n::t;
use sycamore::prelude::*;
use sycamore_router::navigate;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Detail<G: Html>() -> View<G>
{
    let state: AppState = use_context();
    let selected_account = match state
        .selected_account
        .get_clone()
    {
        Some(selected_account) => selected_account,
        None =>
        {
            navigate("/");
            return view! {};
        },
    };
    let account_name = selected_account.account_name.clone();

    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.detail.heading"))
            {
                Box(classes=StrProp("flex flex_column flex_gap_md").into())
                {
                    Heading(variant=HeadingVariant::Bullet.into())
                    {
                        (t!("pages.account.detail.account.heading"))
                    }
                    (t!("pages.account.detail.account.account_name", account_name = selected_account.account_name))
                    Box
                    {
                        (
                            if selected_account.is_public
                            {
                                view! { (t!("pages.account.detail.account.public")) }
                            }
                            else
                            {
                                view! { (t!("pages.account.detail.account.private")) }
                            }
                        )
                    }
                }
                Box(classes=StrProp("flex flex_column flex_gap_md").into())
                {
                    Heading(variant=HeadingVariant::Bullet.into())
                    {
                        (t!("pages.account.detail.profiles.heading"))
                    }
                    AccountProfiles(account_name=account_name)
                    Button
                    (
                        href=OptionProp(Some("/account/profile/create".to_string())).into(),
                        classes=StrProp("flex_align_self_center").into(),
                        round=ButtonRound::Full.into(),
                        left_icon=view!
                        {
                            Icon
                            (
                                icon=IconKind::Plus.into(),
                                color=Colors::PrimaryText.into(),
                            )
                        },
                    )
                    {
                        (t!("pages.account.detail.profiles.button.create"))
                    }
                }
                Box(classes=StrProp("flex flex_column flex_gap_md").into())
                {
                    Heading(variant=HeadingVariant::Bullet.into())
                    {
                        (t!("pages.account.detail.groups.heading"))
                    }
                    Button
                    (
                        href=OptionProp(Some("/account/group/create".to_string())).into(),
                        classes=StrProp("flex_align_self_center").into(),
                        round=ButtonRound::Full.into(),
                        left_icon=view!
                        {
                            Icon
                            (
                                icon=IconKind::Plus.into(),
                                color=Colors::PrimaryText.into(),
                            )
                        },
                    )
                    {
                        (t!("pages.account.detail.groups.button.create"))
                    }
                }
            }
        }
    }
}
