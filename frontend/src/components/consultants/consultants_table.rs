use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component};
use yew::prelude::*;

use common::Consult;

use crate::components::consultants::edit_modal::EditModal;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    // pub specialty: Specialty,
    pub on_load: Callback<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseConsultant {
    pub consultant_id: i32,
    pub specialty_id: i32,
    pub consultant_f_name: String,
    pub consultant_l_name: String,
    pub consultant_slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsultantPostResponse {
    pub consultant_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultantList {
    pub consultants: Vec<ResponseConsultant>,
}

fn render_td_table_rows(list: &Vec<ResponseConsultant>) -> Vec<Html> {
    list.iter()
        .map(|consultant| {
            html! {
            <tr>
                <td>{consultant.consultant_id}</td>
                <td>{consultant.specialty_id.clone()}</td>
                <td>{consultant.consultant_f_name.clone()}</td>
                <td>{consultant.consultant_l_name.clone()}</td>
                <td><button><EditModal id={consultant.consultant_id} button_text={"Edit"} /></button></td>
            </tr>
        }
        })
        .collect()
}

#[styled_component(ConsultantsTable)]
pub fn consultants_table(props: &Props) -> Html {
    // let entity = use_state(|| "consult".to_owned());
    let data: UseStateHandle<Option<Vec<ResponseConsultant>>> = use_state(|| None);
    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            let data_c = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8000/admin/consultants")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<ResponseConsultantList>()
                    .await
                    .unwrap();

                // log!(serde_json::to_string_pretty(&response).unwrap());
                data_c.set(Some(response.consultants))
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
                            <th>{"Specialty"}</th>  
                            <th>{"First Name"}</th>
                            <th>{"Last Name"}</th>
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
