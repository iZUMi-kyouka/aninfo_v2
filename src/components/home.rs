use wasm_bindgen::closure::Closure;
use web_sys::{HtmlDocument, HtmlElement, Window};
use std::collections::HashMap;
use web_sys::js_sys::Date;

use crate::{prelude::*, remove_cookie};

fn apply_horizontal_scroll(win: &Window) {
    let doc = win.document().unwrap();
    let mut last_call_rec = Date::now();
    let last_call_char =  Date::now();

    let s_wrapper = Closure::new(
        {
            let win = win.clone();
            Box::new(
                move |ev: Event| {
                    ev.prevent_default();
                    let doc = win.document().unwrap();
                    if Date::now() > last_call_rec + 0f64 {
                        last_call_rec = Date::now();
                        let scroll = ev.dyn_into::<WheelEvent>().expect("Valid cast.");
                        let rec_wrapper = doc.get_element_by_id("seasonal-cards-wrapper");
                        if let Some(e) = rec_wrapper {
                            log!("scrolling!");
                            e.scroll_by_with_x_and_y(scroll.delta_y()*3.5f64+scroll.delta_x(), 0f64);
                        } else {
                            return ()
                        }
                    }
                }
            ) as Box<dyn FnMut(_)>
        }
    );

    let top_wrapper = Closure::new(
        {
            let doc = doc.clone();
            Box::new(
                move |ev: Event| {
                    ev.prevent_default();
                    let scroll = ev.dyn_into::<WheelEvent>().expect("Valid cast.");
                    let char_wrapper = doc.get_element_by_id("top-cards-wrapper");
                    if let Some(e) = char_wrapper {
                        e.scroll_by_with_x_and_y(scroll.delta_y()*3.5f64+scroll.delta_x(), 0f64);
                        log!("scrolling!");
                    } else {
                        return ()
                    }
                }
            ) as Box<dyn FnMut(_)>
        }

    );

    gloo::timers::callback::Timeout::new(200, move || {
        let wrapper_1 = doc.get_element_by_id("seasonal-cards-wrapper");
        if let Some(e) = wrapper_1 {
            e.dyn_into::<HtmlElement>().expect("Valid cast.").set_onwheel(Some(s_wrapper.as_ref().unchecked_ref()));
        }

        let wrapper_2 = doc.get_element_by_id("top-cards-wrapper");
        if let Some(e) = wrapper_2 {
            e.dyn_into::<HtmlElement>().expect("Valid cast.").set_onwheel(Some(top_wrapper.as_ref().unchecked_ref()));
        }

        s_wrapper.forget();
        top_wrapper.forget();
    }).forget();
}

#[function_component(Content)]
fn content() -> HtmlResult {

    use_effect_once(|| {
        let window = web_sys::window().unwrap();
        apply_horizontal_scroll(&window);
        || {}
    });

    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let nav = use_navigator().unwrap();
    let result: UseFutureHandle<(
        Result<QueryResult, gloo::net::Error>,
        Result<QueryResult, gloo::net::Error>,
    )> = use_future_with(((*cx).language, (*cx).nsfw), |_| async move {
        if let Some(t) = (*cx).cache.home_page_result.clone() {
            (Ok(t.0), Ok(t.1))
        } else {
            let seasonal = force_req::<QueryResult>(&format!(
                "https://api.jikan.moe/v4/seasons/now?sfw={}",
                !(*cx).nsfw
            ))
            .await;
            let top = force_req::<QueryResult>(&format!(
                "https://api.jikan.moe/v4/top/anime?sfw={}",
                !(*cx).nsfw
            ))
            .await;
            if let (Ok(s), Ok(t)) = (seasonal, top) {
                cx.dispatch(
                    (*cx).update_cache_into(
                        (*cx)
                            .cache
                            .update_home_page_result(Some((s.clone(), t.clone()))),
                    ),
                );
                (Ok(s), Ok(t))
            } else {
                (
                    Err(GlooError("".to_string())),
                    Err(GlooError("".to_string())),
                )
            }
        }
    })?;

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
        let time = time.clone();
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::AnimeResultStd {
                content_title: format!("{}'s Selection", &(*time)),
                page: 1,
                url: SEASONAL,
            });
        })
    };

    let go_top_anime = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::AnimeResultStd {
                content_title: "Anime of All Time".to_string(),
                page: 1,
                url: TOP,
            });
        })
    };

    let move_left_seasonal = {
        Callback::from(move |_: MouseEvent| {
            let cards_wrapper = get_elem_by_id!("seasonal-cards-wrapper");
            cards_wrapper.scroll_by_with_x_and_y(-714., 0.);
        })
    };

    let move_right_seasonal = {
        Callback::from(move |_: MouseEvent| {
            let cards_wrapper = get_elem_by_id!("seasonal-cards-wrapper");
            cards_wrapper.scroll_by_with_x_and_y(714., 0.);
        })
    };

    let move_left_top = {
        Callback::from(move |_: MouseEvent| {
            let cards_wrapper = get_elem_by_id!("top-cards-wrapper");
            cards_wrapper.scroll_by_with_x_and_y(-714., 0.);
        })
    };

    let move_right_top = {
        Callback::from(move |_: MouseEvent| {
            let cards_wrapper = get_elem_by_id!("top-cards-wrapper");
            cards_wrapper.scroll_by_with_x_and_y(714., 0.);
        })
    };

    let html_result = match &(*result) {
        (Ok(seasonal), Ok(top)) => {
            let seasonal = seasonal.clone().data;
            let top = top.clone().data;
            html! {
            <>
            <div class={format!("home-wrapper")}>

            <div class={format!("seasonal-wrapper obj-level-1")}>
                <div class="header-home">
                    <a id="l-arrow-s" class={format!("a-btn-bg cursor-pointer nostretch ripple {} obj-level-2", theme)} onclick={move_left_seasonal}><img class={format!("cursor-pointer icon-{} header-arrow", theme)} src="./static/left-arrow.png" width="20px"/></a>
                    <a class="cursor-pointer" onclick={go_to_cur_season.clone()}><h2 class="cursor-pointer content-ttl glow-onhover u_onhover">{format!("{}'s Selection", &(*time))}</h2></a>
                    <a class="cursor-pointer" onclick={go_to_cur_season}><img class={format!("cursor-pointer icon-{}", theme)} src="./static/external.png" id="goto" width="24px"/></a>
                    <a id="r-arrow-s" class={format!("a-btn-bg cursor-pointer nostretch ripple {} obj-level-2", theme)} onclick={move_right_seasonal}><img class={format!("cursor-pointer icon-{} header-arrow", theme)} src="./static/right-arrow.png" width="20px"/></a>
                </div>
                <div id="seasonal-cards-wrapper" class="cards-wrapper b-y">
                {
                    seasonal.into_iter().map(|anime_obj| {
                        html! (<AnimeCard anime_obj={anime_obj.clone()} />)
                    }).collect::<Html>()
                }
                </div>
            </div>

            <div class={format!("top-wrapper obj-level-1")}>
                <div class="header-home">
                    <a id="l-arrow-t" class={format!("a-btn-bg cursor-pointer nostretch ripple {} obj-level-2", theme)} onclick={move_left_top}><img class={format!("cursor-pointer icon-{} header-arrow", theme)} src="./static/left-arrow.png" width="20px"/></a>    
                    <a class="cursor-pointer" onclick={go_top_anime.clone()}><h2 class="cursor-pointer content-ttl glow-onhover u_onhover">{"Anime of All Time"}</h2></a>
                    <a class="cursor-pointer" onclick={go_top_anime}><img class={format!("cursor-pointer icon-{}", theme)} src="./static/external.png" id="goto" width="24px"/></a>
                    <a id="r-arrow-t" class={format!("a-btn-bg cursor-pointer nostretch ripple {} obj-level-2", theme)} onclick={move_right_top}><img class={format!("cursor-pointer icon-{} header-arrow", theme)} src="./static/right-arrow.png" width="20px"/></a>            
                </div>
                <div id="top-cards-wrapper" class="cards-wrapper b-y">
                {
                    top.into_iter().map(|anime_obj| {
                        html! (<AnimeCard anime_obj={anime_obj.clone()} />)
                    }).collect::<Html>()
                }
                </div>
            </div>

            </div>
            </>
            }
        }
        _ => {
            html! {}
        }
    };

    Ok(html!({ html_result }))
}

#[function_component(Home)]
pub fn test_component() -> Html {
    let fallback = html! {<Loading/>};
    use_title("Home | ANiNFO".to_string());
    let cx = use_context::<AppContext>().unwrap();

    html! {
        <>
            <Suspense {fallback}>
                <Content/>
            </Suspense>
        </>
    }
}
