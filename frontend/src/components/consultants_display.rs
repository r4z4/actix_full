use gloo::console::log;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    // pub specialty: Specialty,
    pub on_load: Callback<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultantList {
    pub consultants: Vec<ResponseConsultant>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseConsultant {
    pub consultant_id: i32,
    pub specialty: String,
    pub img_path: Option<String>,
}

// #[derive(PartialEq)]
// pub enum Specialty {
//     Insurance,
//     Finance,
//     Government,
// }

fn vec_to_html(list: &ResponseConsultantList) -> Vec<Html> {
    list.consultants.iter()
        .map(|consultant| {
            html! {<ul class="data-display">
                <li>{consultant.consultant_id.clone()}</li>
                <li>{consultant.specialty.clone()}</li>
                <li>{consultant.img_path.clone()}</li>
                <li>
                    <div>
                        <img src={consultant.img_path.clone()} width={50} height={50} />
                    </div>
                </li>
            </ul>}
        })
        .collect()
}

// impl Entity {
//     pub fn to_string(&self) -> String {
//         match self {
//             Specialty::Government => "Government Entity".to_owned(),
//             Specialty::Finance => "Finance".to_owned(),
//             Specialty::Insurance => "Insurance".to_owned(),
//         }
//     }
// }

#[styled_component(ConsultantsDisplay)]
pub fn consultants_display(props: &Props) -> Html {
    let entity = use_state(|| "consult".to_owned());
    let data: UseStateHandle<Option<ResponseConsultantList>> = use_state(|| None);
    let cloned_data = data.clone();
    let onclick = {
        let entity = entity.clone();
        Callback::from(move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8000/api/consultants")
                    //.header("x-auth-token", &state.token)
                    .send()
                    .await
                    // FIXME unwrap_or_else - handle
                    .unwrap()
                    .json::<ResponseConsultantList>()
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
            if cloned_data.is_some() {
                {vec_to_html(cloned_data.as_ref().unwrap())}
            }
            <button {onclick}>{"Get Data"}</button>
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
