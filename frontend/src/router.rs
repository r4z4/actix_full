use crate::components::pages::consultants::Consultants;
use crate::components::pages::consults::Consults;
use crate::components::pages::clients::Clients;
use crate::components::pages::engagements::Engagements;
use crate::components::pages::home::Home;
use crate::components::pages::inquire::Inquire;
use crate::components::pages::login::Login;
use crate::components::pages::register::Register;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/inquire")]
    Inquire,
    #[at("/consults")]
    Consults,
    #[at("/clients")]
    Clients,
    #[at("/engagements")]
    Engagements,
    #[at("/consultants")]
    Consultants,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Inquire => html! { <Inquire /> },
        Route::Clients => html! { <Clients /> },
        Route::Register => html! { <Register /> },
        Route::Consults => html! { <Consults /> },
        Route::Engagements => html! { <Engagements /> },
        Route::Consultants => html! { <Consultants /> },
    }
}
