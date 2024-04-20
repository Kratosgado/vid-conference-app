// profile.rs
use yew::prelude::*;

use crate::components::profile_text::ProfileText;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div class="profile-container">
            <h1>{ "Profile" }</h1>
            <div class="profile-info">
                <ProfileText label="Username" value="admin" />
                <ProfileText label="Email" value="admin@example.com" />
                <ProfileText label="Bio" value="This is a sample bio" />
            </div>
        </div>
    }
}