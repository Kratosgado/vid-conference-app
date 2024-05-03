// profile.rs
use yew::prelude::*;
use yewdux::use_store;

use crate::{components::profile_text::ProfileText, helpers::states::State};

#[derive(Properties, Clone, PartialEq)]
pub struct ProfileProps {
    pub username: String,
    pub email: String,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let ProfileProps { username, email } = props;
    // log::info!("User: {:?}", user);
    html! {
        <div class="profile-container">
            <h1>{ "Profile" }</h1>
            <div class="profile-info">
                <ProfileText label="Username" value={username.clone()} />
                <ProfileText label="Email" value={email.clone()} />
                <ProfileText label="Bio" value="This is a sample bio" />
            </div>
        </div>
    }
}
