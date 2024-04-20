use yew::prelude::*;
use yew_router::prelude::*;

pub mod route;
use route::Route;
pub mod pages;
pub mod components;

use components::topbar::TopBar;

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
                <TopBar />
                <Switch<Route> render={Route::switch}  />
            </BrowserRouter>
        }
    }
}
fn main(){
    yew::Renderer::<App>::new().render();
}
