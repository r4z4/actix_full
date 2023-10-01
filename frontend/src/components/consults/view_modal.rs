use std::ops::Deref;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::prelude::*;

use crate::components::attachments_display::AttachmentsDisplay;
use super::consults_form::ConsultsForm;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub attachment_ids: Vec<i32>,
    pub button_text: String,
    pub consult_id: i32,
}

#[derive(Properties, Deserialize, Serialize, PartialEq)]
pub struct AttachmentsResponse {
    pub attachments: Vec<AttachmentData>,
}

#[derive(Properties, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct AttachmentData {
    pub url: String,
    pub typ: String,
    pub name: String,
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

#[styled_component(ViewModal)]
pub fn view_modal(props: &Props) -> Html {
    // let entity = use_state(|| "consult".to_owned());
    let attachment_ids = &props.attachment_ids;
    let button_text = &props.button_text;
    let consult_id = &props.consult_id;
    let data: UseStateHandle<Option<Vec<AttachmentData>>> = use_state(|| None);

    let aids = attachment_ids.clone();

    let data_clone = data.clone();

    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data_c = data.clone();
            let url = format!("http://localhost:8000/admin/attachments/{:?}", aids);
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&url)
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<AttachmentsResponse>()
                    .await
                    .unwrap();

                // log!(serde_json::to_string_pretty(&response).unwrap());
                data_c.set(Some(response.attachments))
            });
        })
    };
    html! {
        <div>
            if data.is_some() {
                // FIXME: Ensure only ONE dialog can be open at once
                <dialog open={true} id={"edit_modal"} class="dialog-display">
                <button onclick={|_| close_modal()}>{"Close"}</button>
                <h3>{format!("Attachments for {}", consult_id)}</h3>
                <AttachmentsDisplay attachments_data={data.as_ref().unwrap().clone()} />
                    
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
