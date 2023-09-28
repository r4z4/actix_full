use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component};
use yew::prelude::*;

use common::Consult;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    // pub specialty: Specialty,
    pub on_load: Callback<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseClient {
    pub client_id: i32,
    pub client_address_one: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientPostResponse {
    pub consult_id: i32,
    pub consult_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseClientList {
    pub consults: Vec<ResponseClient>,
}

fn vec_to_html(list: &Vec<ResponseClient>) -> Vec<Html> {
    list.iter()
        .map(|client| {
            html! {
            <div class="entity-display">
                <ul class="display-list">
                    <li>{client.client_id.clone()}</li>
                    <li>{client.client_address_one.clone()}</li>
                </ul>
            </div>
        }
        })
        .collect()
}

#[styled_component(ClientsDisplay)]
pub fn clients_display(props: &Props) -> Html {
    // let entity = use_state(|| "consult".to_owned());
    let data: UseStateHandle<Option<Vec<ResponseClient>>> = use_state(|| None);
    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8000/api/clients")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<Vec<ResponseClient>>()
                    .await
                    .unwrap();

                // log!(serde_json::to_string_pretty(&response).unwrap());
                data.set(Some(response))
            });
        })
    };
    props.on_load.emit("Data Display Loaded".to_string());
    html! {
        <div class={"data-display"}>
            <h1>{&props.title}</h1>
            <h4>{"Click Below to Fetch Data"}</h4>
            if data.is_some() {
                {vec_to_html(data.as_ref().unwrap())}
            }
            <button {onclick}>{
                if data.is_none() {
                    "Get Data"
                } else {
                    "Refresh Data"
                }
            }
            </button>
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
