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
    pub input_type: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
}

#[function_component]
pub fn RequiredTextInput(props: &Props) -> Html {
    let placeholder = &props.placeholder;
    let class = &props.class;
    let input_type = &props.input_type;
    let label = &props.label;
    let name = &props.name;
    let value = if props.value.is_some() {
        props.value.clone().unwrap()
    } else {
        "".to_owned()
    };
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
        <div class={if name == "re_password" {"input-div"} else {"slim-input-div"}}>
            <label for="select">{label}</label>
            <input
                type={input_type.to_owned()}
                ref={text_input_ref}
                name={name.clone()}
                value={value}
                class={class.clone()}
                oninput={on_input_change}
                placeholder={placeholder.clone()}
                required={true}
            />
        </div>
    }
}