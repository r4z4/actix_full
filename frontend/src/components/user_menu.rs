use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};
use yewdux::prelude::use_store;

use crate::router::Route;
use crate::store::AuthStore;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[styled_component(UserMenu)]
pub fn user_menu(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<AuthStore>();

    html! {
        <div class="dropdown">
            <span><img width={30px} src={"/img/anon.svg"} /></span>
            <div class="dropdown-content">
                <p><Link<Route> to={Route::UserProfile}>{"Profile 🧑"}</Link<Route>></p>
                <hr />
                <p>{"Settings ⚙"}</p>
            </div>
      </div>
    }
}
