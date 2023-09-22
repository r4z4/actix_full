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
    pub name: String,
    #[prop_or_default]
    pub date: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<String>,
}


#[function_component]
pub fn DateInput(props: &Props) -> Html {
    let date = props.date.clone();
    let label = &props.label;
    let name = &props.name;
    let cloned_name = name.clone();

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<String>().unwrap();
        onchange.emit(selected)
    });


    html! {
        <div class={"input-div"}>
            <label for="start">{label}</label>
            <input type="date" id="start" name={cloned_name} onchange={on_input_change.clone()} value={date} min="2023-01-01" max="2025-12-31" />
        </div>
    }
}
