use crate::router::{switch, Route};
use yew::prelude::*;
use yew_bootstrap::util::{include_cdn, include_cdn_js};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
          {include_cdn()}
          <BrowserRouter>
            <Switch<Route> render={switch} />
          </BrowserRouter>
          {include_cdn_js()}
        </>
    }
}
