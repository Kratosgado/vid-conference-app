use yew::prelude::*;

pub struct Button {
    props: Props,
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onclick: Callback<()>,
    pub label: String,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Button {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!("Render the button")
    }
}