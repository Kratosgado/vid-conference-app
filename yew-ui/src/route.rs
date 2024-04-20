use yew::prelude::*;
use yew_router::Routable;

use crate::pages::{
    login::Login,
    home::Home,
    signup::Signup,
    profile::Profile,
};
use crate::components::topbar::TopBar;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/login")] Login,
    #[at("/")] Home,
    #[at("/signup")] Signup,
    #[at("/meeting1")] Meeting1,
    #[at("/meeting2")] Meeting2,
    #[at("/settings")] Settings,
    #[at("/profile")] Profile,
}

impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Login => html! { <Login /> },
            Route::Meeting1 => todo!(),
            Route::Meeting2 => todo!(),
            Route::Settings => todo!(),
            Route::Home => html! { 
            <>
                <TopBar />
                <Home />
            </> },
            Route::Signup => html! { <Signup /> },
            // Route::Meeting1 => html! { <pages::meeting1::Meeting1 /> },
            // Route::Meeting2 => html! { <pages::meeting2::Meeting2 /> },
            // Route::Settings => html! { <pages::settings::Settings /> },
            Route::Profile => html! { <Profile /> },
        }
    }
}