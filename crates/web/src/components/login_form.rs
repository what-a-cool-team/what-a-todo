use yew::prelude::*;
use log::info;
use web_sys::HtmlInputElement;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);

    let on_username_input = {
        let username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();

            info!("Username: {}", username);
            info!("Password: {}", password);

            //TODO: make a login API call to get a token.
        })
    };

    html! {
        <section>
          <div class="container mt-5 pt-5">
            <div class="row">
              <div class="col-12 col-sm-7 col-md-4 m-auto">
                <div class="card border-0 shadow">
                  <div class="card-body">
                    <center>
                      <svg class="mx-auto my-3" xmlns="http://www.w3.org/2000/svg" width="50" height="50" fill="currentColor" class="bi bi-person-circle" viewBox="0 0 16 16">
                        <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z" />
                        <path fill-rule="evenodd" d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z" />
                      </svg>
                    </center>
                    <form {onsubmit}>
                      <input
                        type="text"
                        id="username"
                        class="form-control my-4 py-2"
                        placeholder="Username"
                        value={(*username).clone()}
                        oninput={on_username_input}
                      />
                      <input
                        type="password"
                        id="password"
                        class="form-control my-4 py-2"
                        placeholder="Password"
                        value={(*password).clone()}
                        oninput={on_password_input}
                      />
                      <div class="text-center mt-3">
                        <button type="submit" class="btn btn-primary">{"Login"}</button>
                      </div>
                      <div class="text-center mt-3">
                        <a href="#" class="nav-link p-0 d-inline-block">{"Forgot password?"}</a>
                      </div>
                      <div class="text-center mt-3">
                        <a href="#" class="nav-link p-0 d-inline-block">{"Not a user?"}</a>
                      </div>
                    </form>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
    }
}
