use yew::prelude::*;
use log::info;
use web_sys::HtmlInputElement;

fn create_input_handler(value_state: UseStateHandle<String>, touched_state: UseStateHandle<bool>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();
        value_state.set(value.clone());
        touched_state.set(!value.is_empty());
    })
}

#[function_component(SignupForm)]
pub fn signup_form() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let confirm_password = use_state(String::new);
    let show_password = use_state(|| false);

    // Track whether fields have been touched (modified at least once)
    let username_touched = use_state(|| false);
    let password_touched = use_state(|| false);
    let confirm_password_touched = use_state(|| false);

    // Validation states
    let username_valid = username.len() > 3;
    let password_valid = password.len() >= 4;
    let passwords_match = *password == *confirm_password && password_valid;

    // Check if the entire form is valid
    let form_valid = username_valid && password_valid && passwords_match;

    // Create input handlers
    let on_username_input = create_input_handler(username.clone(), username_touched.clone());
    let on_password_input = create_input_handler(password.clone(), password_touched.clone());
    let on_confirm_password_input = create_input_handler(confirm_password.clone(), confirm_password_touched.clone());

    // Toggle password visibility
    let on_toggle_password_visibility = {
        let show_password = show_password.clone();
        Callback::from(move |_| {
            show_password.set(!*show_password);
        })
    };

    // Form submission handler
    let onsubmit = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();

            if form_valid {
                info!("Username: {}", username);
                info!("Password: {}", password);
                // TODO: make an API call to register the user.
            } else {
                info!("Form is not valid, cannot submit.");
            }
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
                        class={classes!("form-control", "mt-4",
                          if *username_touched && username_valid { "is-valid" }
                          else if *username_touched && !username_valid { "is-invalid" }
                          else { "" }
                        )}
                        placeholder="Username"
                        value={(*username).clone()}
                        oninput={on_username_input}
                      />
                      if *username_touched && !username_valid {
                        <div class="invalid-feedback mt-2 mb-3">
                          {"Username must be more than 3 characters"}
                        </div>
                      }

                      <input
                        type={if *show_password { "text" } else { "password" }}
                        id="password"
                        class={classes!("form-control", "mt-4",
                          if *password_touched && password_valid { "is-valid" }
                          else if *password_touched && !password_valid { "is-invalid" }
                          else { "" }
                        )}
                        placeholder="Password (at least 4 characters)"
                        value={(*password).clone()}
                        oninput={on_password_input}
                      />
                      if *password_touched && !password_valid {
                        <div class="invalid-feedback mt-2 mb-3">
                          {"Password must be at least 4 characters"}
                        </div>
                      }

                      <input
                        type={if *show_password { "text" } else { "password" }}
                        id="confirm_password"
                        class={classes!("form-control", "mt-4",
                          if *confirm_password_touched && passwords_match { "is-valid" }
                          else if *confirm_password_touched && !passwords_match { "is-invalid" }
                          else { "" }
                        )}
                        placeholder="Confirm Password"
                        value={(*confirm_password).clone()}
                        oninput={on_confirm_password_input}
                      />
                      if *confirm_password_touched && !passwords_match {
                        <div class="invalid-feedback mt-2 mb-3">
                          {"Passwords do not match"}
                        </div>
                      }

                      <div class="form-check my-3">
                        <input
                          type="checkbox"
                          class="form-check-input"
                          id="show_password"
                          checked={*show_password}
                          onclick={on_toggle_password_visibility}
                        />
                        <label class="form-check-label" for="show_password">
                          {"Show Passwords"}
                        </label>
                      </div>

                      <div class="text-center mt-3">
                        <button type="submit" class="btn btn-primary" disabled={!form_valid}>{"Sign Up"}</button>
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
