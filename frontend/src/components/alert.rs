#![allow(non_camel_case_types)]

use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::{set_hide_alert, Store};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub message: String,
    pub typ: i32,
    pub delay_ms: u32,
}

fn get_typ_class(typ: i32) -> String {
    match typ {
        1 => "alert-success".to_owned(),
        2 => "alert-failure".to_owned(),
        _ => "".to_owned(),
    }
}

#[function_component]
pub fn AlertComponent(props: &Props) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let show_alert = store.alert_input.show_alert;

    use_effect_with_deps(
        move |(show_alert, dispatch, delay_ms)| {
            let cloned_dispatch = dispatch.clone();
            if *show_alert {
                let handle =
                    Timeout::new(*delay_ms, move || set_hide_alert(cloned_dispatch)).forget();
                let clear_handle = move || {
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap(),
                        handle.as_f64().unwrap() as i32,
                    );
                };

                Box::new(clear_handle) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        },
        (show_alert, dispatch.clone(), props.delay_ms),
    );

    html! {
    <div id="myToast" class={format!("my-toast {}", if show_alert { "" } else { "hidden" })}>
        <p class={format!("toast-p {}", get_typ_class(props.typ))}>
            <span class="toast-span">{"i"}</span>
            {props.message.clone()}
        </p>
    </div>
    }
}
