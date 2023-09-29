use std::ops::Deref;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component};
use yew::prelude::*;

use common::Consult;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub button_text: String,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseClient {
    pub client_id: i32,
    pub client_address_one: String,
    pub client_city: String,
    pub client_zip: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientPostResponse {
    pub client_id: i32,
    pub consult_zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseClientList {
    pub clients: Vec<ResponseClient>,
}

#[styled_component(EditModal)]
pub fn edit_modal(props: &Props) -> Html {
    // let entity = use_state(|| "consult".to_owned());
    let button_text = &props.button_text;
    let id = props.id;
    let data: UseStateHandle<Option<ResponseClient>> = use_state(|| None);
    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data_c = data.clone();
            let url = format!("http://localhost:8000/api/clients/{}", id);
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&url)
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<ResponseClient>()
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
                <dialog open={true} class="dialog-display">
                    <p>{"Greetings, one and all!"}</p>
                    <form method="dialog">
                        <h3>{format!("Form for {}", data.deref().clone().unwrap().client_id)}</h3>
                        <button>{"OK"}</button>
                    </form>
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
