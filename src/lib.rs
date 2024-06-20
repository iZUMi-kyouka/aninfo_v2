mod components;
mod routers;
mod stores;
mod utils;

pub mod prelude {
    // pub const BASE_URL: &'static str = "http://localhost:8000";
    pub const BASE_URL: &'static str = "https://aninfo-server.shuttleapp.rs";
    pub const JIKAN_URL: &'static str = "https://api.jikan.moe/v4/anime";
    pub use chrono::prelude::*;
    pub use std::hash::{DefaultHasher, Hash, Hasher};
    pub use std::{fmt::Display, str::FromStr};
    pub use yew::prelude::*;
    pub use yew::suspense::*;
    pub use yew_hooks::prelude::*;
    pub use yew_router::prelude::*;
    pub use yewdux::prelude::*;
    // pub use nyaa_si::model::Torrent;

    pub use web_sys::wasm_bindgen::JsCast;
    pub use web_sys::{DomRectReadOnly, HtmlCollection};

    pub use serde::{Deserialize, Serialize};
    pub use serde_aux::prelude::*;
    pub use serde_json::to_string_pretty;

    pub use gloo::console::log;
    pub use gloo::net::Error::{GlooError, JsError, SerdeError};

    pub use tokio::join;

    pub use crate::components::appctx_handler::*;

    pub use crate::components::about::*;
    pub use crate::components::anime_card::*;
    pub use crate::components::anime_details::*;
    pub use crate::components::anime_result_std::*;
    pub use crate::components::backup::*;
    pub use crate::components::character_card::*;
    pub use crate::components::comment_section::*;
    pub use crate::components::context_debugger::*;
    pub use crate::components::episode_cards::*;
    pub use crate::components::error::*;
    pub use crate::components::experiment::*;
    pub use crate::components::explore::*;
    pub use crate::components::footer::*;
    pub use crate::components::header::*;
    pub use crate::components::header_nb_wrapper::*;
    pub use crate::components::home::*;
    pub use crate::components::left_navbar::*;
    pub use crate::components::loading::*;
    pub use crate::components::page_button::*;
    pub use crate::components::privacy_policy::*;
    pub use crate::components::search_result::*;
    pub use crate::components::test::*;
    pub use crate::components::torrent_popup::*;
    pub use crate::components::torrents::*;

    pub use crate::stores::genre_list::*;
    pub use crate::stores::*;

    pub use crate::routers::*;

    pub(crate) use crate::utils::app_macros::get_elem_by_id;
    pub use crate::utils::data_handling::*;
    pub use crate::utils::general::*;
    pub use crate::utils::interface::*;
    pub use crate::utils::into_html::*;
}

use std::collections::HashMap;

use components::login_wrapper::LoginWrapper;
use cookie::{time::OffsetDateTime, Cookie, CookieBuilder, Expiration};
use time::{Duration, Time, UtcOffset};
use wasm_bindgen::closure::Closure;
use web_sys::{Document, Element, HtmlDocument, HtmlElement, HtmlInputElement, HtmlTextAreaElement, Window};

use self::prelude::*;

fn apply_search_shortcut(win: &Window) {
    let d = win.document().unwrap();
    let c = Closure::new(
        Box::new(
            move |source_e: Event| {
                let key = source_e
                    .clone()
                    .dyn_into::<KeyboardEvent>()
                    .unwrap();
                let search_field = d.get_element_by_id("nb-search").unwrap().unchecked_into::<HtmlInputElement>();
                let key_pressed = key.key().to_lowercase();
                // log!(&format!("Key {} pressed.", &key_pressed));
                let alt_key = key.alt_key();

                // Check whether there is any active INPUT element
                let active_element = match d.active_element() {
                    None => None,
                    Some(e) => {
                        match e.clone().dyn_into::<HtmlInputElement>() {
                            Ok(ref e) => {
                                let elem_name = e.tag_name();
                                match (e.type_().to_lowercase().as_str(), elem_name.to_lowercase().as_str()) {
                                    ("text", "input") => Some(()),
                                    _ => None
                                }
                            },
                            Err(_) => {
                                if let Ok(e) = e.dyn_into::<HtmlTextAreaElement>() {
                                    Some(())
                                } else {
                                    None
                                }
                            }
                        }
                    }             
                };

                match (key_pressed.as_str(), alt_key, active_element) {
                    ("q", _, None) =>  {
                        log!("q pressed.");
                        if let Some(_) = d.get_element_by_id("main-header") {
                            let search_field = d.get_element_by_id("nb-search").unwrap().unchecked_into::<HtmlInputElement>();
                            search_field.focus().expect("Must be able to focus on search field.");
                            search_field.select();
                            source_e.prevent_default();
                        } else {
                            d.get_element_by_id("main-header-hidden").unwrap().set_id("main-header");
                            let search_field = d.get_element_by_id("nb-search").unwrap().unchecked_into::<HtmlInputElement>();
                            search_field.focus().expect("Must be able to focus on search field.");
                            search_field.select();
                            source_e.prevent_default();
                        }
                    },
                    ("x", _, None) => {
                        log!("x is pressed.");
                        if let Some(e) = d.active_element() {
                            log!("got active elem.");
                            if e.tag_name().to_lowercase() == "body" {
                                log!("no active input. toggling theme.");
                                if let Some(e) = d.get_element_by_id("theme-toggle-icon") {
                                    e.dyn_into::<HtmlElement>().unwrap().click();
                                }
                            } else {
                                if let Some(event) = source_e.target() {
                                    if let Ok(elem) = event.dyn_into::<HtmlInputElement>() {
                                        match (elem.tag_name().to_lowercase().as_str(), elem.type_().as_str()) {
                                            ("input", "text") | ("input", "textarea") => {
                                                // Do nothing
                                                log!("an input is detected.");
                                            },
                                            _ => {
                                                log!("no active input. toggling theme.");
                                                source_e.prevent_default();
                                                if let Some(e) = d.get_element_by_id("theme-toggle-icon") {
                                                    e.dyn_into::<HtmlElement>().unwrap().click();
                                                }
                                            }
                                        }
                                    }
                                }
                            } 
                        } 
                    },
                    _ => {}
                }
            }
        ) as Box<dyn FnMut(_)>
    );

    win.document().unwrap().set_onkeydown(Some(c.as_ref().unchecked_ref()));
    c.forget()
}

fn hide_header_onscroll(win: &Window) {
    let doc = win.document().unwrap();
    let d = win.document().unwrap();
    let w = win.clone();

    let mut prev_scroll_pos = Box::new(win.scroll_y().unwrap());
    let c = Closure::wrap(
        Box::new(move |e: Event| {
            let nb = d.get_element_by_id("nb-left").unwrap();
            if nb.get_attribute("class").unwrap().contains("nb-left-active") {
                return ()
            }
            // log!("Callback fired!");
            let cur_scroll_pos = w.scroll_y().unwrap();
            if cur_scroll_pos > *prev_scroll_pos {
                if let Some(e) = d.get_element_by_id("main-header") {
                    e.set_id("main-header-hidden")
                }
            } else {
                if let Some(e) = d.get_element_by_id("main-header-hidden") {
                    e.set_id("main-header")
                }
            }
            *prev_scroll_pos = cur_scroll_pos;
            // log!(&format!("ScrollPos: {}", cur_scroll_pos));
        }
        ) as Box<dyn FnMut(_)>
    );

    doc.set_onscroll(Some(c.as_ref().unchecked_ref()));
    c.forget();
}

pub fn set_cookie(name: &str, value: &str, path: &str, max_age: i64) {
    let time = Utc::now() + chrono::Days::new(7);
    let year = format!("{}", time.format("%Y")).trim().parse::<i32>().unwrap();
    let month = format!("{}", time.format("%B"));
    let d = format!("{}", time.format("%e"));
    log!(d);
    let day = format!("{}", time.format("%e")).trim().parse::<u8>().unwrap();
    let cookie = Cookie::build((name.trim(), value))
        .max_age(time::Duration::days(max_age))
        .same_site(cookie::SameSite::Strict)
        .path(path)
        .build()
        .to_string();

    gloo::console::log!("set_cookie: Cookie constructed -> {}", &cookie);
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    match  document.set_cookie(&cookie) {
        Ok(_) => gloo::console::log!("set_cookie: Cookie {} successfully stored!", &cookie),
        Err(err) => gloo::console::error!("set_cookie: Failed to store cookie: {}", err)
    }
}

pub fn remove_cookie(name: &str) {
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    match document.cookie() {
        Ok(s) => {
            set_cookie(name, "", "", 0);
        },
        Err(e) => log!(e)
    }
}


#[function_component(App)]
pub fn app() -> Html {
    // use_before_unload(true, "Reloading will log you out of your acccount. Proceed to reload?".to_string());
    use_effect_once(|| {
        let win = web_sys::window().unwrap();
        hide_header_onscroll(&win);
        apply_search_shortcut(&win);
        || {}
    });


    html! {
        <>
            <AppContextProvider>
                <LoginWrapper>
                    <BrowserRouter>
                        <HNBWrapper>
                            <LeftNavbar/>
                            <Header/>
                        </HNBWrapper>
                        <Switch<Route> render={switch}/>
                        <Footer/>
                    </BrowserRouter>
                </LoginWrapper>
            </AppContextProvider>
        </>
    }
}
