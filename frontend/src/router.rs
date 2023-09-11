// use crate::components::pages::home::Home;
// use crate::components::pages::login::Login;
// use crate::components::pages::register::Register;
use crate::components::pages::consults::Consults;
use crate::components::pages::engagements::Engagements;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    // #[at("/")]
    // Home,
    // #[at("/login")]
    // Login,
    // #[at("/register")]
    // Register,
    #[at("/consults")]
    Consults,
    #[at("/engagements")]
    Engagements,
}

pub fn switch(route: Route) -> Html {
    match route {
        // Route::Home => html! { <Home /> },
        // Route::Login => html! { <Login /> },
        // Route::Register => html! { <Register /> },
        Route::Consults => html! { <Consults /> },
        Route::Engagements => html! { <Engagements /> },
    }
}
