use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
          <p>{ "Hello, TODO!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
