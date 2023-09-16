use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub color: String,
}

#[function_component(Nav)]
pub fn nav(props: &Props) -> Html {
    html! {
        <div>
            <ul id={"nav-list"}>
                <li><Link<Route> to={Route::Consults}>{"Consults"}</Link<Route>></li>
                <li><Link<Route> to={Route::Engagements}>{"Engagements"}</Link<Route>></li>
                <li><Link<Route> to={Route::Consultants}>{"Consultants"}</Link<Route>></li>
                <li><Link<Route> to={Route::Home}>{"🏠"}</Link<Route>></li>
            </ul>
        </div>
    }
}
