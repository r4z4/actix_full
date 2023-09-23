use std::ops::Deref;

use common::{SelectOption, SelectOptionResponse};
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub onchange: Callback<Option<String>>,
}

#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let placeholder = &props.placeholder;
    let label = &props.label;
    let text_input_ref = use_node_ref();
    // let state = use_state_eq(|| None);
    // let cloned_state = state.clone();

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: InputEvent| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        onchange.emit(Some(value))
    });

    html! {
        <div class={"input-div"}>
            <label for="select">{label}</label>
            <input
                type="text"
                ref={text_input_ref}
                oninput={on_input_change}
                class="text-input"
                placeholder={placeholder.clone()}
            />
        </div>
    }
}