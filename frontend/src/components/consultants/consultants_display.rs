use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
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
    pub specialty_id: i32,
    pub img_path: Option<String>,
}

fn vec_to_html(list: &Vec<ResponseConsultant>) -> Vec<Html> {
    list.iter()
        .map(|consultant| {
            html! {
            <div class="entity-display">
                <ul class="display-list">
                    <li>{consultant.consultant_id.clone()}</li>
                    <li>{consultant.specialty_id.clone()}</li>
                    <li>{consultant.img_path.clone()}</li>
                    <li>
                        <div>
                            <img src={consultant.img_path.clone()} width={50} height={50} />
                        </div>
                    </li>
                </ul>
            </div>
        }
        })
        .collect()
}


#[styled_component(ConsultantsDisplay)]
pub fn consults_display(props: &Props) -> Html {
    let entity = use_state(|| "consult".to_owned());
    let data: UseStateHandle<Option<Vec<ResponseConsultant>>> = use_state(|| None);
    let onclick = {
        let data = data.clone();
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
                data.set(Some(response.consultants))
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
