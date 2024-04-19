use yew::prelude::*;
use yew_router::prelude::*;


struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {}
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(());
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        // info!("OAuth enabled: {}", *ENABLE_OAUTH);
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch}  />
            </BrowserRouter>
        }
    }
}
fn main(){
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <App /> },
        Route::Secure => html! { <Secure /> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

