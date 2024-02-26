//------------------------------------------------------------------------------
//! Account profile card.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct AccountProfileCardProps
{
    pub account_name: String,
    pub avatar_file_key: String,
    pub cover_file_key: String,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn AccountProfileCard<G: Html>( props: AccountProfileCardProps ) -> View<G>
{
    view!
    {
        ProfileCover(file_key=OptionProp(Some(props.cover_file_key)).into())
        ProfileAvatar(file_key=OptionProp(Some(props.avatar_file_key)).into())
        (props.account_name)
    }
}
