use chrono::{DateTime, Utc};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::{ConsultStore, set_consults};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub on_load: Callback<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseConsult {
    pub consult_id: i32,
    pub location_id: i32,
    // #[serde(serialize_with = "serialize_dt", skip_serializing_if  = "Option::is_none")]
    pub consult_start: Option<DateTime<Utc>>,
    pub consult_attachments: Option<String>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsultPostResponse {
    pub consult_id: i32,
    pub consult_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultList {
    pub consults: Vec<ResponseConsult>,
}

fn vec_to_html(list: &Vec<ResponseConsult>) -> Vec<Html> {
    list.iter()
        .map(|consult| {
            html! {
            <div class="entity-display">
                <ul class="display-list">
                    <li>{consult.consult_id.clone()}</li>
                    <li>{consult.location_id.clone()}</li>
                    <li>{consult.notes.clone()}</li>
                    <li>{consult.consult_start.unwrap()}</li>
                    <li>{consult.consult_attachments.clone().unwrap()}</li>
                </ul>
            </div>
        }
        })
        .collect()
}

#[styled_component(ConsultsDisplay)]
pub fn consults_display(props: &Props) -> Html {
    let entity = use_state(|| "consult".to_owned());
    let data: UseStateHandle<Option<Vec<ResponseConsult>>> = use_state(|| None);
    let (store, dispatch) = use_store::<ConsultStore>();
    let c_data = data.clone();

    use_effect_with_deps(
        move |_| {
            ()
        },
        (c_data),
    );

    let onclick = {
        let entity = entity.clone();
        let data = data.clone();
        Callback::from(move |_| {
            let c_data = data.clone();
            let dispatch_clone = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8000/api/consults")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<ResponseConsultList>()
                    .await
                    .unwrap();

                // log!(serde_json::to_string_pretty(&response).unwrap());
                let consults_clone = response.consults.clone();
                c_data.set(Some(response.consults));
                // set_consults(consults_clone, dispatch_clone);
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
