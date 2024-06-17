use crate::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loading-container">
            <progress class="pure-material-progress-circular"/>
        </div>

    }
}
