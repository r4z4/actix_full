use std::ops::Deref;

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
    let data: UseStateHandle<Option<ResponseConsult>> = use_state(|| None);
    // let addr_state: UseStateHandle<String> = use_state(|| String::from(""));
    // let city_state: UseStateHandle<String> = use_state(|| String::from(""));
    // let zip_state: UseStateHandle<String> = use_state(|| String::from(""));
    // let error_state = use_state(|| None);

    let data_clone = data.clone();
    let f_name_changed: Callback<String> = Callback::from(move |f_name| {
        // Move this inside so it clones the data in there
        let cloned_state: UseStateHandle<Option<ResponseConsult>> = data_clone.clone();
        let resp = ResponseConsult {
            consult_id: data_clone.deref().clone().unwrap().consult_id,
            location_id: data_clone.deref().clone().unwrap().location_id,
            consult_start: data_clone.deref().clone().unwrap().consult_start,
            consult_attachments: None,
            notes: data_clone.deref().clone().unwrap().notes,
        };
        cloned_state.set(Some(resp));
    });

    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data_c = data.clone();
            let url = format!("http://localhost:8000/admin/consults/{}", id);
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&url)
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<ResponseConsult>()
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
                <h3>{format!("Form for {}", data.deref().clone().unwrap().consult_id)}</h3>
                <img src={format!("/img/consults/consult_{}.svg", data.deref().clone().unwrap().consult_id)} />
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
