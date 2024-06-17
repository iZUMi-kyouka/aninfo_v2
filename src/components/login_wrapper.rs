use std::collections::HashMap;

use web_sys::HtmlDocument;
use yew::prelude::*;
use crate::prelude::*;

use crate::remove_cookie;

#[derive(Properties, Debug, PartialEq)]
pub struct LoginWrapperProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(LoginWrapper)]
pub fn login_wrapper(props: &LoginWrapperProps) -> Html {
    let cx = use_context::<AppContext>().unwrap();

    {
        let cx = cx.clone();
        use_effect_with(((*cx).jwt.clone(), (*cx).username.clone(), (*cx).uuid.clone()), move |_| {
            let closure_to_return = || {};
            let cx = cx.clone();
            // Check if user data is present, and if user is authorized
            if (*cx).jwt.is_some() && (*cx).username.is_some() && (*cx).uuid.is_some() {
                return closure_to_return;
            }
            let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
            let cookie = document.cookie().unwrap_or_default();
            let cookies = cookie
                .split(';')
                .map(|s| {
                    let mut cookie = s.trim().split('=');
                    let key = cookie.next().unwrap_or_default().trim();
                    let value = cookie.next().unwrap_or_default().trim();
                    (key, value)
                } )
                .collect::<HashMap<&str, &str>>();
            log!(&format!("login-wrapper: Cookies found on device -> {:#?}", cookies));

            match cookies.get("userdata") {
                None => { // Cookie not found
                    log!("login-wrapper: No JWT cookie found.");
                },
                Some(jwt) => { // User cookie is found, check if cookie is valid
                    log!("login-wrapper: JWT cookie found: ", *jwt);
                    cx.update_jwt_into(Some(jwt.to_string()));

                    {
                        let cx = cx.clone();
                        let jwt = jwt.to_string();
                        wasm_bindgen_futures::spawn_local(async move {
                            let link = format!("{}/api/v1/login_check", BASE_URL);
                            let result = reqwasm::http::Request::post(&link)
                                .header("content-type", "application/json")
                                .header("Authorization", &format!("Bearer {}", jwt))
                                .send()
                                .await;

                            match result {
                                Err(_) => { // Any JWT stored in cookie is invalid, remove from memory if any
                                    cx.dispatch((*cx).update_jwt_into(None).update_username_into(None).update_uuid_into(None));
                                    remove_cookie("userdata");
                                },
                                Ok(response) => { // JWT stored in cookie is valid, proceed to update user data in memory
                                    let user = response.json::<UserForResponse>().await;
                                    if let Ok(user) = user {
                                        log!(&format!("Setting user info: {:?}", user));
                                        cx.dispatch(
                                            (*cx)
                                                .update_jwt_into(Some(jwt.to_string()))
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
                                    }
                                    // set_cookie("userdata", &jwt[7..], "/")
                                }
                            }
                        });
                    }
                }
            };

            closure_to_return
        });
    }

    html!{
        <>
        {props.children.clone()}
        </>
    }
}