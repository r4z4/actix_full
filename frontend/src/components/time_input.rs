use std::ops::Deref;

use common::{SelectOption, SelectOptionResponse};
use gloo_console::log;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub time: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<String>,
}


#[function_component]
pub fn TimeInput(props: &Props) -> Html {
    // let time: UseStateHandle<String> = use_state(|| "00:00".to_owned());
    let time = props.time.clone();
    let label = &props.label;
    let name = &props.name;
    let cloned_name = name.clone();

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<String>().unwrap();
        log!(selected.clone());
        onchange.emit(selected)
    });


    html! {
        <div class={"input-div"}>
            <label for="start">{label}</label>
            <input type="time" id="start" name={cloned_name} onchange={on_input_change.clone()} value={time} min="09:00" max="18:00" />
            <small>{"Office hours are 9am to 6pm"}</small>
        </div>
    }
}
