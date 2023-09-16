use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        <button>{&props.label}</button>
    }
}
