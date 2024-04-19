use yew::prelude::*;
use yew_router::Routable;

use crate::Login;

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
            Route::Home => todo!(),
            Route::Signup => todo!(),
            Route::Meeting1 => todo!(),
            Route::Meeting2 => todo!(),
            Route::Settings => todo!(),
            Route::Profile => todo!(),
            // Route::Home => html! { <pages::home::Home /> },
            // Route::Signup => html! { <pages::signup::Signup /> },
            // Route::Meeting1 => html! { <pages::meeting1::Meeting1 /> },
            // Route::Meeting2 => html! { <pages::meeting2::Meeting2 /> },
            // Route::Settings => html! { <pages::settings::Settings /> },
            // Route::Profile => html! { <pages::profile::Profile /> },
        }
    }
}