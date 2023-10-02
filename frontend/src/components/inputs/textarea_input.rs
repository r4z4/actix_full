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
    pub class: String,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
}

#[function_component]
pub fn TextAreaInput(props: &Props) -> Html {
    let placeholder = &props.placeholder;
    let label = &props.label;
    let class = &props.class;
    let value = 
        if props.value.is_some() {
            props.value.clone().unwrap()
        } else {
            "".to_owned()
        };
    let name = &props.name;
    let text_input_ref = use_node_ref();
    // let state = use_state_eq(|| None);
    // let cloned_state = state.clone();

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: InputEvent| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        onchange.emit(value)
    });

    html! {
        <div class={"input-div"}>
            <label for="notes_textarea">{label}</label>
            <input
                type="textarea"
                id={"notes_textarea"}
                ref={text_input_ref}
                name={name.clone()}
                oninput={on_input_change}
                class={class}
                value={value}
                placeholder={placeholder.clone()}
            />
        </div>
    }
}