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
    pub selected: u8,
    #[prop_or_default]
    pub select_type: String,
    #[prop_or_default]
    pub onchange: Callback<i32>,
}

// trait SelectOption {
//     fn select_html(&self) {
//         for option in &self {
//             option_html.push(
//             html! {
//                 <option value={option.value}>{ option.key }</option>
//             }
//         )
//         };
//     };
// }

// #[derive(Clone, PartialEq)]
// pub struct SelectOption {
//     key: String,
//     value: String,
// }

#[function_component]
pub fn SelectInput(props: &Props) -> Html {
    let selected = props.selected;
    let label = &props.label;
    let select_type = props.select_type.clone();
    let route = 
        match select_type.as_ref() {
            "location" => "http://localhost:8000/api/location-options",
            "consultant" => "http://localhost:8000/api/consultant-options",
            "client" => "http://localhost:8000/api/client-options",
            "account" => "http://localhost:8000/api/account-options",
            _ => "",
        };
    let state = use_state_eq(|| None);
    let cloned_state = state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::get(route)
            //.header("x-auth-token", &state.token)
            .send()
            .await
            // FIXME unwrap_or_else - handle
            .unwrap()
            .json::<SelectOptionResponse>()
            .await
            .unwrap();

        // log!(serde_json::to_string_pretty(&response).unwrap());
        match response.status.as_ref() {
            "success" => cloned_state.set(Some(response.options)),
            "fail" => cloned_state.set(None),
            _ => cloned_state.set(None),
        }
    });

    let location_options: Vec<SelectOption> = vec![
        SelectOption {
            key: "location_one".to_string(),
            value: 1,
        },
        SelectOption {
            key: "location_two".to_string(),
            value: 2,
        },
    ];
    let mut option_html = vec![];
    let cloned_state = state.clone();
    let data = cloned_state.deref().clone();
    if let Some(options) = data {
        if let Some(option) = options {
            for opt in option {
                option_html.push(html! {
                    <option value={opt.value.to_string()}>{ opt.key }</option>
                })
            }
        }
    }
    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<i32>().unwrap();
        onchange.emit(selected)
    });

    html! {
        <div class={"input-div"}>
            <label for="select">{label}</label>
            <select id="select" onchange={on_input_change}>
                {option_html}
            </select>
        </div>
    }
}
