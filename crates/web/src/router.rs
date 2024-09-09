use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::signup::SignUp;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    SignUp,
    #[at("/login")]
    Login,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::SignUp => html! {
            <SignUp />
        },
        Route::Login => html! {
            <Login />
        }
    }
}
