// profile.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfileTextProps {
    pub label: String,
    pub value: String,
}

#[function_component(ProfileText)]
pub fn profile_text(props: &ProfileTextProps) -> Html {
    html! {
        <div class="profile-text">
            <div class="label">{ props.label.clone() }</div>
            <div class="value">{ props.value.clone() }</div>
        </div>
    }
}