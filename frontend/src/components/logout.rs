use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::router::Route;
use crate::store::AuthStore;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[styled_component(Logout)]
pub fn logout(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<AuthStore>();
    let user_logout = Callback::from(move |_| {
        // Clear store
        dispatch.reduce_mut(|store| store.token = None);
        dispatch.reduce_mut(|store| store.username = None);
        navigator.push(&Route::Home);
    });
    html! {
        <div>
            <button class={"logout"} onclick={user_logout}>{&props.label}</button>
        </div>
    }
}
