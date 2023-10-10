use std::ops::Deref;

use chrono::{DateTime, Utc};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::prelude::*;

use super::{consults_form::ConsultsForm, consults_display::ResponseConsult};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub button_text: String,
    pub id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsultPostResponse {
    pub consult_id: i32,
    pub consult_zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultList {
    pub consults: Vec<ResponseConsult>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct FormConsultEdit {
    pub consultant_id: i32,
    pub client_id: i32,
    pub location_id: i32,
    // #[serde(serialize_with = "serialize_dt", skip_serializing_if  = "Option::is_none")]
    pub consult_start: DateTime<Utc>,
    pub consult_end: Option<DateTime<Utc>>,
    // CASTing to varchar
    pub consult_attachments: Option<Vec<i32>>,
    pub notes: Option<String>,
}

fn close_modal() -> () {
    let window = web_sys::window()
        .unwrap();
    let document = window
        .document()
        .unwrap();
    let modal: HtmlDialogElement = document.get_element_by_id("edit_modal").unwrap().unchecked_into::<HtmlDialogElement>();
    modal.close();
}

#[styled_component(EditModal)]
pub fn edit_modal(props: &Props) -> Html {
    // let entity = use_state(|| "consult".to_owned());
    let button_text = &props.button_text;
    let id = props.id;
    let data: UseStateHandle<Option<FormConsultEdit>> = use_state(|| None);
    // let addr_state: UseStateHandle<String> = use_state(|| String::from(""));
    // let city_state: UseStateHandle<String> = use_state(|| String::from(""));
    // let zip_state: UseStateHandle<String> = use_state(|| String::from(""));
    // let error_state = use_state(|| None);

    let data_clone = data.clone();
    let f_name_changed: Callback<String> = Callback::from(move |f_name| {
        // Move this inside so it clones the data in there
        let cloned_state: UseStateHandle<Option<FormConsultEdit>> = data_clone.clone();
        let resp = FormConsultEdit {
            consultant_id: data_clone.deref().clone().unwrap().consultant_id,
            client_id: data_clone.deref().clone().unwrap().client_id,
            location_id: data_clone.deref().clone().unwrap().location_id,
            // These need Option<>
            consult_start: data_clone.deref().clone().unwrap().consult_start,
            consult_end: data_clone.deref().clone().unwrap().consult_end,
            consult_attachments: None,
            notes: data_clone.deref().clone().unwrap().notes,
        };
        cloned_state.set(Some(resp));
    });

    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data_c = data.clone();
            let url = format!("http://localhost:8000/api/consults/form/{}", id);
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&url)
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<FormConsultEdit>()
                    .await
                    .unwrap();

                // log!(serde_json::to_string_pretty(&response).unwrap());
                data_c.set(Some(response))
            });
        })
    };
    html! {
        <div>
            if data.is_some() {
                // FIXME: Ensure only ONE dialog can be open at once
                <dialog open={true} id={"edit_modal"} class="dialog-display">
                    <button onclick={|_| close_modal()}>{"Close"}</button>
                    <h3>{format!("Form for {}", id)}</h3>
                    <img src={format!("/img/consults/consult_{}.svg", id)} />
                    <ConsultsForm data={data.deref().clone().unwrap()} />
                </dialog >
            }
            <button {onclick}>{button_text}</button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    // Bring things into scope
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
