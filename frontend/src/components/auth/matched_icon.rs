use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub state: Option<bool>,
}

#[function_component(MatchedIcon)]
pub fn matched_icon(props: &Props) -> Html {
    
    html! {
        if props.state.is_some() {
            if props.state.unwrap() == true {
                <icon class={"matched-icon text-green"}>{"✅ Passwords Match"}</icon>
            } else {
                <icon class={"matched-icon text-red"}>{"❌ Passwords Do Not Match"}</icon>
            }
        }
    }
}