use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::logout::Logout;
use crate::components::user_menu::UserMenu;
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
                <li><Link<Route> to={Route::Clients}>{"Clients"}</Link<Route>></li>
                <li><Link<Route> to={Route::Home}><img width={30px} src={"img/home.svg"} /></Link<Route>></li>
                <li><UserMenu label={"label"} /></li>
                <li><Logout label={"->"}/></li>
            </ul>
        </div>
    }
}
