use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component};
use yew::prelude::*;

use common::Consult;

use crate::components::clients::edit_modal::EditModal;

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

fn render_td_table_rows(list: &Vec<ResponseClient>) -> Vec<Html> {
    list.iter()
        .map(|client| {
            html! {
            <tr>
                <td>{client.client_id}</td>
                <td>{client.client_address_one.clone()}</td>
                <td>{client.client_city.clone()}</td>
                <td>{client.client_zip.clone()}</td>
                <td><button><EditModal id={client.client_id} button_text={"Edit"} /></button></td>
            </tr>
        }
        })
        .collect()
}

#[styled_component(ClientsTable)]
pub fn clients_table(props: &Props) -> Html {
    // let entity = use_state(|| "consult".to_owned());
    let data: UseStateHandle<Option<Vec<ResponseClient>>> = use_state(|| None);
    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data_c = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8000/api/clients")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<ResponseClientList>()
                    .await
                    .unwrap();

                // log!(serde_json::to_string_pretty(&response).unwrap());
                data_c.set(Some(response.clients))
            });
        })
    };
    props.on_load.emit("Data Display Loaded".to_string());
    html! {
        <div class={"data-display"}>

            if data.is_some() {
                <div class="table-display">
                    <table border="1" >
                        <tr>
                            <th>{"ID"}</th>
                            <th>{"Address"}</th>  
                            <th>{"City"}</th>
                            <th>{"Zip"}</th>
                            <th>{"Action(s)"}</th>
                        </tr>
                        {render_td_table_rows(data.as_ref().unwrap())}
                    </table>
                </div>
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
