use web_sys::HtmlInputElement;
use std::collections::HashMap;
use web_sys::HtmlDocument;

use crate::{prelude::*, remove_cookie};

#[function_component(Header)]
pub fn header() -> Html {
    let cx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let theme = handle_theme(&cx);
    let query = use_state(|| "".to_string());

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

    // let go_to_about = {
    //     let nav = nav.clone();
    //     Callback::from(move |e: MouseEvent| {
    //         web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
    //         nav.push(&Route::About);
    //     })
    // };

    // let go_to_exp = {
    //     let nav = nav.clone();
    //     Callback::from(move |e: MouseEvent| {
    //         nav.push(&Route::Experiment);
    //     })
    // };

    // let go_to_debug = {
    //     let nav = nav.clone();
    //     Callback::from(move |e: MouseEvent| {
    //         nav.push(&Route::Debug);
    //     })
    // };
    let time = use_state(|| "".to_string());

    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let cur_time = Utc::now();
            let year = format!("{}", cur_time.format("%Y"));
            let season = season_from_month(&format!("{}", cur_time.format("%m")));
            time.set(format!("{} {}", season, year));
        });
    }

    let go_to_cur_season = {
        let nav = nav.clone();
        let time = time.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::AnimeResultStd {
                content_title: format!("{}'s Selection", &(*time)),
                page: 1,
                url: SEASONAL,
            });
        })
    };

    let go_to_explore = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::ExploreAnime);
        })
    };

    let input_query = {
        let query = query.clone();
        Callback::from(move |e: InputEvent| {
            // log!("query input!");
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            query.set(value);
        })
    };

    let search_anime = {
        let query = query.clone();
        let nav = nav.clone();
        let cx = cx.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); //Why is it that if .update_page_into is dispatched in separate line, causes bug?
            if !(*query).is_empty() {
                // log!("Non empty query detected. Fetching...");
                cx.dispatch(
                    (*cx)
                        .update_query_into((*query).clone())
                        .update_page_into(1),
                );
                web_sys::window()
                    .expect("Missing window.")
                    .scroll_to_with_x_and_y(0f64, 0f64);
                nav.push(&Route::SearchResultNoPage {
                    q: (*query).clone(),
                });
            }
        })
    };

    let open_left_nb = {
        let theme = theme.clone();
        move |_| {
            let left_nb = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("nb-left")
                .unwrap();
            let menu_icon = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("menu-icon")
                .unwrap();
            let cur_class = left_nb.get_attribute("class").unwrap();
            if cur_class.contains("nb-left-active") {
                left_nb
                    .set_attribute("class", &format!("nb-left {}", theme.clone()))
                    .unwrap();
                menu_icon
                    .set_attribute("class", &format!("icon-{} rotate-half", theme))
                    .unwrap();
                let menu_icon = menu_icon.clone();
                gloo::timers::callback::Timeout::new(75, move || {
                    menu_icon.set_attribute("src", "./static/menu.svg").unwrap();
                })
                .forget();
            } else {
                left_nb
                    .set_attribute(
                        "class",
                        &format!("nb-left nb-left-active {}", theme.clone()),
                    )
                    .unwrap();
                menu_icon
                    .set_attribute("class", &format!("icon-{} rotate-ref", theme))
                    .unwrap();
                let menu_icon = menu_icon.clone();
                gloo::timers::callback::Timeout::new(75, move || {
                    menu_icon
                        .set_attribute("src", "./static/close_nb.png")
                        .unwrap();
                })
                .forget();
            }
        }
    };

    html! {
        <div id="main-header" class={format!("header {}", theme)}>
        <ul class="header-section hs-left">
        <li><a class={"a-btn ripple"} onclick={open_left_nb}><img id="menu-icon" class={format!("icon-{}", theme)} src="./static/menu.svg" width="28px" height="28px" /></a></li>
            <li><a class="a-btn" onclick={move |e: MouseEvent| {
                e.prevent_default();
                nav.clone().push(&Route::HomeNew);
            }}><h2 class="main-logo cursor-pointer">{"ANiNFO"}</h2></a></li>
            <li><a class="a-btn ripple hideable" onclick={go_to_cur_season}><img id="cur-season-icon" class={format!("icon-{}", theme)} src="./static/season.png" width="28px" height="28px" /></a></li>
            <li><a class="a-btn ripple" onclick={go_to_explore}><img id="explore-icon" class={format!("icon-{}", theme)} src="./static/explore2.png" width="28px" height="28px" /></a></li>
            // <li><a class="a-btn ripple able" onclick={go_to_about}><img id="about-icon" class={format!("icon-{}", theme)} src="./static/about.svg" width="28px" height="28px" /></a></li>
            // <li><a class="a-btn ripple hideable" onclick={go_to_exp}><img id="about-icon" class={format!("icon-{}", theme)} src="./static/about.svg" width="28px" height="28px" /></a></li>
            // <li><a class="a-btn ripple hideable" onclick={go_to_debug}><img id="about-icon" class={format!("icon-{}", theme)} src="./static/about.svg" width="28px" height="28px" /></a></li>
        </ul>

        <ul class="header-section hs-right">

            <li>
                <form id="query" onsubmit={search_anime.clone()} method="post">
                    <input id="nb-search" class={format!("nb-query {}", theme)} type="text" oninput={input_query} placeholder="Type `Q` to search"/>
                    <input type="submit" class="hidden" id="submit-query"/>
                </form>
            </li>

            // <li><article class="nsfw-tog-wrapper"><input id="nsfw-tog" type="checkbox"/>
            // <label id="nsfw-tog-label" for="nsfw-tog">{"NSFW"}</label></article></li>
            <li><label for="submit-query"><a class="a-btn ripple" type="submit" form="query"><img id="theme-toggle-icon" class={format!("icon-{}", theme)} src="./static/search.svg" width="28px" height="28px" /></a></label></li>
            // <li><button class="cursor-pointer" type="submit" onsubmit={search_anime} id="search"><img class={format!("ripple icon-{}", theme)} id="search-icon" width="28px" height="28 px" src="./static/search.svg" alt="search--v1"/></button></li>
            <li><a class="a-btn ripple hideable" onclick={change_theme}><img id="theme-toggle-icon" class={format!("icon-{}", theme)} src={handle_theme_icon(&cx)} width="28px" height="28px" /></a></li>
            <li><a class="a-btn ripple hideable" onclick={change_language}><img id="lang-toggle-icon" class={format!("icon-{}", theme)} src="./static/lang.png" width="28px" height="28px" /></a></li>
        </ul>
    </div>
    }
}
