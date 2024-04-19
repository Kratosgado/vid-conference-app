use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    let navigator = use_navigator().unwrap();

    let navigate = |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&route);
        })
    };
    
    html!(
        <div class="flex justify-between items-center py-4 bg-gray-100">
            <div class="flex items-center">
                <h1 class="text-2xl font-bold">{ "App Name" }</h1>
            </div>
            <div class="flex items-center">
                <ul class="flex items-center">
                    <li class="mr-4">
                        <a href="#" onclick={navigate(Route::Home)}>{"Home"}</a>
                    </li>
                    <li class="mr-4">
                        <a href="#" onclick={navigate(Route::Meeting1)}>{"Meeting 1"}</a>
                    </li>
                    <li class="mr-4">
                        <a href="#" onclick={navigate(Route::Meeting2)}>{"Meeting 2"}</a>
                    </li>
                    <li class="mr-4">
                        <a href="#" onclick={navigate(Route::Settings)}>{"Settings"}</a>
                    </li>
                    <li class="mr-4">
                        <a href="#" onclick={navigate(Route::Profile)}>{"Profile"}</a>
                    </li>
                </ul>
            </div>
        </div>
    )
}