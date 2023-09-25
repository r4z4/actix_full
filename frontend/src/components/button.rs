use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub class: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let class = &props.class;
    html! {
        <button class={class}>{&props.label}</button>
    }
}
