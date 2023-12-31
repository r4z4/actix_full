use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub date: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<String>,
}


#[function_component]
pub fn DateInput(props: &Props) -> Html {
    let date = props.date.clone();
    let label = &props.label;
    let required = props.required;
    let name = &props.name;
    let cloned_name = name.clone();

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<String>().unwrap();
        onchange.emit(selected)
    });


    html! {
        <div class={"input-div"}>
            <label for="start">{label}</label>
            <span class={"datepicker-toggle"}>
                <span class={"datepicker-toggle-button"}></span>
                <input type="date" class={"datepicker-input"} name={cloned_name} onchange={on_input_change.clone()} value={date} min="2023-01-01" max="2025-12-31" required={required}/>
            </span>
        </div>
    }
}
