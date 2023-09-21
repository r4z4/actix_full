use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub selected: u8,
    #[prop_or_default]
    pub onchange: Callback<u8>,
}

pub struct LocationOption {
    key: String,
    value: String,
}



#[function_component]
pub fn LocationSelect(props: &Props) -> Html {
    let selected = props.selected;

    let location_options: Vec<LocationOption> = vec![LocationOption {key: "location_one".to_string(), value: '1'.to_string()}, LocationOption {key: "location_two".to_string(), value: '2'.to_string()}];
    let mut option_html = vec![];               
    for option in location_options {
        option_html.push(
        html! {
            <option value={option.value}>{ option.key }</option>
        }
    )
    };
    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<u8>().unwrap();
        onchange.emit(selected)
    });

    html! {
        <div>
            <select>
                {option_html}
            </select>
        </div>
    }
}