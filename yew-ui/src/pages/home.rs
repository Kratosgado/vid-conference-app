use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex justify-center items-center content-center flex-col m-auto">
            <div class="flex items-center flex-col">
                <h1>{ "Home" }</h1>
            </div> 
            <p>{ "Welcome to the home page!" }</p>
        </div>
    }
}