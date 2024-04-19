use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    html!(
        <div>
            <p>{ *counter }</p>
            <button {onclick}>{ "Increment" }</button>
        </div>
    )
}

fn main(){
    yew::Renderer::<App>::new().render();
}