use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component};
use yew::prelude::*;

use common::Consult;

use crate::components::consults::{edit_modal::EditModal, view_modal::ViewModal, consults_display::{ResponseConsult, ResponseConsultList}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    // pub specialty: Specialty,
    pub on_load: Callback<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsultPostResponse {
    pub consult_id: i32,
}

fn get_attachments(attachment_ids: Option<Vec<i32>>) -> Vec<i32> {
    if let Some(attachments) = attachment_ids {
        attachments
    } else {
        vec![]
    }
}

fn render_td_table_rows(list: &Vec<ResponseConsult>) -> Vec<Html> {
    list.iter()
        .map(|consult| {
            html! {
            <tr>
                <td>{consult.consult_id}</td>
                <td>{consult.location_id.clone()}</td>
                <td>{consult.consult_start.clone()}</td>
                <td>{consult.notes.clone()}</td>
                // <td><button><ViewModal attachment_ids={get_attachments(consult.consult_attachments.clone())} button_text={"Edit"} consult_id={consult.consult_id} /></button></td>
                <td><button><ViewModal attachment_ids={consult.consult_attachments.clone()} button_text={"Edit"} consult_id={consult.consult_id} /></button></td>
                <td><button><EditModal id={consult.consult_id} button_text={"Edit"} /></button></td>
            </tr>
        }
        })
        .collect()
}

#[styled_component(ConsultsTable)]
pub fn consults_table(props: &Props) -> Html {
    // let entity = use_state(|| "consult".to_owned());
    let data: UseStateHandle<Option<Vec<ResponseConsult>>> = use_state(|| None);
    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data_c = data.clone();
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
                data_c.set(Some(response.consults))
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
                            <th>{"Location ID"}</th>  
                            <th>{"Start Time"}</th>
                            <th>{"Notes"}</th>
                            <th>{"Attachments"}</th>
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
