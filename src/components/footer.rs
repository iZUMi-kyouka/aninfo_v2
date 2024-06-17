use crate::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let cx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let theme = handle_theme(&cx);

    let back_to_top = |_: MouseEvent| {
        let window = web_sys::window().unwrap();
        window.scroll_to_with_x_and_y(0f64, 0f64);
    };

    html! {
        <div class={format!("footer {}", handle_theme(&cx))}>
        <div class="footer-left">
        <p>{"Made with "}<span class="love-footer">{"❤"}</span>{" by"}<b>{" iZUMi"}</b></p>
        <p id="pp-link"><a class="u_onhover" onclick={move |_| {
            web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
            nav.push(&Route::PrivacyPolicy);
        }}>{"Privacy Policy"}</a>{" | Best viewed using "}<a class="u_onhover" href="https://www.mozilla.org/en-US/firefox/new/">{"Mozilla Firefox."}</a></p>
        </div>

        <div class="footer-right">
            <a class={format!("a-btn ripple {}", "")} onclick={back_to_top}><img id="top-logo" width="30px" height="30px" class={format!("icon-{}", theme)} src="./static/top.svg"/></a>
        </div>
        </div>
    }
}
