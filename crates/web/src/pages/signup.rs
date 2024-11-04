use yew::prelude::*;
use crate::components::signup_form::SignupForm;

#[function_component(SignUp)]
pub fn signup() -> Html {
    html! {
        <SignupForm />
    }
}
