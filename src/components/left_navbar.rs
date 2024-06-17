use std::collections::HashMap;

use gloo::dialogs::alert;
use web_sys::{HtmlDocument, HtmlInputElement};

use crate::{prelude::*, remove_cookie, set_cookie};

#[function_component(LeftNavbar)]
pub fn left_navbar() -> Html {
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let nav = use_navigator().unwrap();
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let server_error = use_state(|| false);
    let password_shown = use_state(|| false);
    
    let change_theme = {
        let cx = cx.clone();
        Callback::from(move |_: MouseEvent| {
            let cur_theme = cx.theme.clone();
            match cur_theme {
                Theme::Light => {
                    cx.dispatch(cx.update_theme_into(DARK_THEME));
                    let html_element = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_elements_by_tag_name("html")
                        .get_with_index(0)
                        .unwrap();
                    html_element.set_attribute("class", "dark").unwrap();
                }
                Theme::Dark => {
                    cx.dispatch(cx.update_theme_into(LIGHT_THEME));
                    let html_element = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_elements_by_tag_name("html")
                        .get_with_index(0)
                        .unwrap();
                    html_element.set_attribute("class", "light").unwrap();
                }
                _ => {}
            };
        })
    };

    let change_language = {
        let cx = cx.clone();
        Callback::from(move |_: MouseEvent| {
            let cur_lang = cx.language.clone();
            match cur_lang {
                Language::EN => {
                    cx.dispatch(cx.update_language_into(Language::JP));
                    let html_element = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_elements_by_tag_name("html")
                        .get_with_index(0)
                        .unwrap();
                    html_element.set_attribute("lang", "ja").unwrap();
                }
                Language::JP => {
                    cx.dispatch(cx.update_language_into(Language::EN));
                    let html_element = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_elements_by_tag_name("html")
                        .get_with_index(0)
                        .unwrap();
                    html_element.set_attribute("lang", "en").unwrap();
                }
            };
        })
    };

    let username_oninput = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            username.set(value);
        })
    };

    let password_oninput = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            password.set(value);
        })
    };

    let form_state = use_state(|| "login".to_string());
    let password_verify = use_state(|| "".to_string());

    let password_ok = use_state(|| false);

    let password_verify_oninput = {
        let password_verify = password_verify.clone();
        let password = password.clone();
        let password_ok = password_ok.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            password_verify.set(value.clone());
        })
    };

    {
        let password = password.clone();
        let p = password.clone();
        let p_v = password_verify.clone();
        let password_verify = password_verify.clone();
        let password_ok = password_ok.clone();
        use_effect_update_with_deps(
            move |_| {
                password_ok.set(verify_password(&(*&password), &(*&password_verify)));
                || {}
            },
            (p, p_v),
        )
    }

    let password_wrong = use_state(|| false);

    let login = {
        let server_error = server_error.clone();
        let password_wrong = password_wrong.clone();
        let cx = cx.clone();
        let (username, password) = (username.clone(), password.clone());
        Callback::from(move |e: SubmitEvent| {
            let password_wrong = password_wrong.clone();
            let server_error = server_error.clone();
            e.prevent_default();
            let cx = cx.clone();
            let (username, password) = (username.clone(), password.clone());
            wasm_bindgen_futures::spawn_local(async move {
                let link = format!("{}/api/v1/users/login", BASE_URL);
                let result = reqwasm::http::Request::post(&link)
                    .body(
                        serde_json::to_string(&UserRequest {
                            username: (*username).clone(),
                            password_hash: sha256::digest((*password).clone()),
                        })
                        .unwrap(),
                    )
                    .header("content-type", "application/json")
                    .send()
                    .await;

                match result {
                    Err(_) => server_error.set(true),
                    Ok(response) => {
                        server_error.set(false);
                        let jwt = response.headers().get("authorization");

                        match jwt {
                            None => {
                                log!("password wrong.");
                                password_wrong.set(true)
                            }
                            Some(jwt) => {
                                log!("password correct!");
                                password_wrong.set(false);
                                let user = response.json::<UserForResponse>().await.unwrap();
                                cx.dispatch(
                                    (*cx)
                                        .update_jwt_into(Some((&jwt[7..]).to_string()))
                                        .update_username_into(Some(user.username))
                                        .update_uuid_into(Some(user.uuid))
                                        .update_fav_anime_into(Some(user.fav_anime.clone()))
                                        .update_fav_anime_id_into(Some(
                                            {
                                                let mut v = user.fav_anime.iter().map(|ua_response| ua_response.anime_id).collect::<Vec<i32>>();
                                                v.sort();
                                                v
                                            }
                                        ))
                                );
                                set_cookie("userdata", &jwt[7..], "/", 7);
                            }
                        }
                    }
                }
            });
        })
    };

    let register = {
        let form_state = form_state.clone();
        let (username, password) = (username.clone(), password.clone());
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form_state = form_state.clone();
            let (username, password) = (username.clone(), password.clone());
            wasm_bindgen_futures::spawn_local(async move {
                let link = format!("{}/api/v1/users/register", BASE_URL);
                let result = reqwasm::http::Request::post(&link)
                    .body(
                        serde_json::to_string(&UserRequest {
                            username: (*username).clone(),
                            password_hash: sha256::digest((*password).clone()),
                        })
                        .unwrap(),
                    )
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap();

                form_state.set("login".to_string());
                alert("Your account has been created! Please login to access all ANiNFO features.");
            });
        })
    };

    let logout = {
        let cx = cx.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let cx = cx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let link = format!("{}/api/v1/users/logout", BASE_URL);
                let auth_header = format!("Bearer {}", (*cx).jwt.as_ref().unwrap());
                let result = reqwasm::http::Request::post(&link)
                    .header("content-type", "application/json")
                    .header("authorization", &auth_header)
                    .send()
                    .await
                    .unwrap();

                cx.dispatch(
                    (*cx)
                        .update_jwt_into(None)
                        .update_username_into(None)
                        .update_uuid_into(None)
                        .update_fav_anime_into(None)
                        .update_fav_anime_id_into(None),
                );
                remove_cookie("userdata");
            });
        })
    };

    let handle_nsfw = {
        let cx = cx.clone();

        Callback::from(move |_: MouseEvent| {
            // log!("NSFW toggled.");
            // e.prevent_default();
            let msg = r#"
Attention for younger audiences

By checking the "Allow NSFW results" option, you acknowledge that anime search results may include content unsuitable for younger viewers. This includes themes, images, or depictions that some may find inappropriate or offensive.

Please be aware that accessing such content may be illegal in certain countries or jurisdictions. Users are solely responsible for any legal repercussions that may arise from their decision to continue with this option enabled. Kindly exercise caution regarding the viewing of such content in various settings.

This feature is strictly intended for users 18 years of age and older. 

For a safer browsing experience, consider keeping the "Allow NSFW results" option unchecked.

*NOTE: All images are retrieved from MyAnimeList's CDN (Content Delivery Network), and all images are censored.
            "#;

            let checkbox = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("nsfw-tog")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            if !(*cx).nsfw {
                let user_ok = web_sys::window()
                    .unwrap()
                    .confirm_with_message(msg)
                    .unwrap();

                if user_ok {
                    checkbox.set_checked(true);
                    checkbox.set_attribute("checked", "true").unwrap();
                    cx.dispatch((*cx).update_nsfw_into(true));
                } else {
                    checkbox.set_checked(false);
                    checkbox
                        .remove_attribute("checked")
                        .expect("Cannot modify checkbox.");
                }
            } else {
                checkbox.set_checked(false);
                checkbox
                    .remove_attribute("checked")
                    .expect("Cannot modify checkbox.");
                cx.dispatch((*cx).update_nsfw_into(false));
            }
        })
    };

    let show_password = {
        let password_shown = password_shown.clone();
        Callback::from(move |_: MouseEvent| {
            // Check current `password shown` state
            if *password_shown == false {
                password_shown.set(true);
            } else {
                password_shown.set(false);
            }

            let elem = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("password")
                .unwrap();
            let password_field: HtmlInputElement = web_sys::window().unwrap().document().unwrap().get_element_by_id("password").unwrap().dyn_into::<HtmlInputElement>().unwrap();
            match *password_shown {
                true => {
                    password_field.set_attribute("type", "password");
                },
                false => {
                    password_field.set_attribute("type", "text");
                }
            }
        })
    };

    let show_password_register = {
        let password_shown = password_shown.clone();
        Callback::from(move |_: MouseEvent| {
            if *password_shown == false {
                password_shown.set(true);
            } else {
                password_shown.set(false);
            }

            let elem1 = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("password")
                .unwrap();
            let elem2 = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("password_verify")
                .unwrap();
            match *password_shown {
                true => {
                    elem1.set_attribute("type", "password");
                    elem2.set_attribute("type", "password");
                },
                false => {
                    elem1.set_attribute("type", "text");
                    elem2.set_attribute("type", "text");
                }
            }
        })
    };

    let cur_greeting = use_state(|| "".to_string());

    {
        let cur_greeting = cur_greeting.clone();
        use_effect_once(move || {
            let time_now = chrono::Local::now().hour();
            if time_now < 12 {
                cur_greeting.set("Good morning".to_string())
            } else if time_now < 17 {
                cur_greeting.set("Good afternoon".to_string())
            } else if time_now < 21 {
                cur_greeting.set("Good evening".to_string())
            } else {
                cur_greeting.set("Good night".to_string())
            }

            || {}
        });
    }

    let register_open = {
        let form_state = form_state.clone();
        Callback::from(move |_: MouseEvent| {
            form_state.set("register".to_string());
        })
    };

    let login_open = {
        let form_state = form_state.clone();
        Callback::from(move |_: MouseEvent| {
            form_state.set("login".to_string());
        })
    };

    let register_open = {
        let form_state = form_state.clone();
        Callback::from(move |_: MouseEvent| {
            form_state.set("register".to_string());
        })
    };

    html! {
        <div id="nb-left" class={format!("nb-left {}", theme)}>
            <div class="nb-items">
                <div class="nb-item flex-row">
                <a class="a-btn ripple only-small-screen" onclick={change_theme}><img id="theme-toggle-icon" class={format!("icon-{}", theme)} src={handle_theme_icon(&cx)} width="28px" height="28px" /></a>
                <a class="a-btn ripple only-small-screen" onclick={change_language}><img id="lang-toggle-icon" class={format!("icon-{}", theme)} src="./static/lang.png" width="28px" height="28px" /></a>
                </div>
                    // LOGIN FORM
                {
                    match (*cx).jwt {
                        None => {
                            // log!("JWT not found in memory.");
                            html!{
                            <>
                            <div class="nb-item">
                            <h3>{&*cur_greeting}{"!"}</h3>
                            {"You are not logged in."}
                            </div>

                            {match &(*form_state).as_str() {
                                &"login" => html! {
                                        <>
                                    <div class="nb-item">
                                        <form id="login-form" onsubmit={login} method="post">
                                            <input class={format!("nb-query {}", theme)} type="text" oninput={username_oninput} placeholder="Username"/>
                                            {
                                                match *password_shown {
                                                    true => {
                                                        html!{<input id="password" class={format!("nb-query {}", theme)} type="text" oninput={password_oninput} placeholder="Password"/>}
                                                    },
                                                    false => {
                                                        html!{<input id="password" class={format!("nb-query {}", theme)} type="password" oninput={password_oninput} placeholder="Password"/>}
                                                    }
                                                }
                                            }
                                           
                                            <input type="submit" class="hidden" id="login"/>
                                        </form>
                                    </div>

                                    <div class="nb-item">
                                    <label class="basic-checkbox-label" for="show-password">
                                        <input id="show-password" class="basic-checkbox" type="checkbox" onclick={show_password}/>
                                        <span class="basic-checkbox-text">{"Show password"}</span>
                                    </label>
                                    </div>

                                    {match (*password_wrong) {
                                        true => {html!{
                                            <span class="warning-plaintext">{"Username and/or password are invalid. Please check your credentials again."}</span>
                                        }},
                                        false => {html!{}}
                                    }}

                                    {match (*server_error) {
                                        true => {html!{
                                            <span class="warning-plaintext">{"Error connecting to the server. Please try logging in again."}</span>
                                        }},
                                        false => {html!{}}
                                    }}

                                    <div class={format!("nb-item nb-max-width")}>
                                        <label for="login">
                                        <a class={format!("card-btn hover-highlight cursor-pointer obj-level-2 {}", &theme)} type="submit" form="login-form">
                                            {"Login"}
                                            // <img id="theme-toggle-icon" class={format!("icon-{}", theme)} src="./static/search.svg" width="28px" height="28px" />
                                        </a>
                                        </label>
                                    </div>


                                    <div id="open-register-button" class="nb-item">
                                        {"Don't have an account yet? "}
                                        <a class="nb-btn cursor-pointer u_onhover" onclick={register_open}>
                                            {"Register!"}
                                            // <img id="theme-toggle-icon" class={format!("icon-{}", theme)} src="./static/search.svg" width="28px" height="28px" />
                                        </a>
                                    </div>
                                    </>
                                },
                                &"register" => html! {
                                    <>
                                    <div class="nb-item">
                                        <form id="registration-form" onsubmit={register} method="post">
                                            <input class={format!("nb-query {}", theme)} type="text" oninput={username_oninput} placeholder="Username"/>
                                            {
                                                match *password_shown {
                                                    true => {
                                                        html!{
                                                            <>
                                                            <input id="password" class={format!("nb-query {}", theme)} type="text" oninput={password_oninput} placeholder="Password"/>
                                                            <input id="password_verify" class={format!("nb-query {}", theme)} type="text" oninput={password_verify_oninput} placeholder="Verify password"/>
                                                            </>
                                                        }
                                                    },
                                                    false => {
                                                        html!{
                                                            <>
                                                            <input id="password" class={format!("nb-query {}", theme)} type="password" oninput={password_oninput} placeholder="Password"/>
                                                            <input id="password_verify" class={format!("nb-query {}", theme)} type="password" oninput={password_verify_oninput} placeholder="Verify password"/>
                                                            </>
                                                        }
                                                    }
                                                }
                                            }
                                            <input type="submit" class="hidden" id="register"/>
                                        </form>
                                    </div>

                                    <div class="nb-item">
                                    <label class="basic-checkbox-label" for="show-password">
                                        <input id="show-password" class="basic-checkbox" type="checkbox" onclick={show_password_register}/>
                                        <span class="basic-checkbox-text">{"Show password"}</span>
                                    </label>
                                    </div>

                                    <div class="nb-item">
                                        <span><b>{"Password Guidelines"}</b></span>
                                        <ol>
                                            <li>{"Password must at least be 8 characters long."}</li>
                                            <li>{"Password must contain at least one of the following: uppercase letter, lowercase letter, symbol/punctuation, and number."}</li>
                                            <li>{"Password must consist of latin characters only."}</li>
                                        </ol>
                                    </div>

                                    <div class="nb-item">
                                        {match *password_ok {
                                            true => html! {},
                                            false => html!{
                                                <span class="warning-plaintext">{"Passwords do not match. Please check the passwords you entered again."}</span>
                                            }
                                        }}
                                    </div>

                                    <div class={format!("nb-item nb-max-width")}>
                                        <label for="register">
                                        <a class={format!("card-btn hover-highlight cursor-pointer obj-level-2 {}", &theme)} type="submit" form="registration-form">
                                            {"Register"}
                                            // <img id="theme-toggle-icon" class={format!("icon-{}", theme)} src="./static/search.svg" width="28px" height="28px" />
                                        </a>
                                        </label>
                                    </div>


                                    <div id="open-register-button" class="nb-item">
                                        {"Already have an account? "}
                                        <a class="nb-btn cursor-pointer u_onhover" onclick={login_open}>
                                            {"Login!"}
                                            // <img id="theme-toggle-icon" class={format!("icon-{}", theme)} src="./static/search.svg" width="28px" height="28px" />
                                        </a>
                                    </div>
                                    </>
                                },
                                _ => html!{}
                            }}
                                </>
                        }},
                        Some(_) => {
                            // log!("JWT found in memory");
                            html!{
                                <>
                                <div class="nb-item">
                                <h3>{&*cur_greeting}<b>{", "}{((cx.username).as_ref().unwrap())}</b>{"!"}</h3>
                                </div>

                                <div class={format!("nb-item nb-max-width")}>
                                <a class={format!("card-btn hover-highlight cursor-pointer obj-level-2 {}", &theme)} onclick={logout}>
                                    {"Logout"}
                                    // <img id="theme-toggle-icon" class={format!("icon-{}", theme)} src="./static/search.svg" width="28px" height="28px" />
                                </a>
                            </div>
                                </>
                            }
                        }
                    }
                }

                <div class="nb-item">
                <label class="basic-checkbox-label" for="nsfw-tog">
                    <input class="basic-checkbox" type="checkbox" id="nsfw-tog" onclick={handle_nsfw}/>
                    <span class="basic-checkbox-text">{"Allow NSFW results"}</span>
                </label>
                </div>

                <div class="nb-item">
                        <a class="a-btn u_onhover" onclick={move |_| nav.push(&Route::About)}>{"About us"}</a>
                </div>

            </div>
        </div>
    }
}
