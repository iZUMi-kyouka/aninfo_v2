use crate::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub q: String,
    pub page: u32,
}

#[function_component(Content)]
fn content(props: &Props) -> HtmlResult {
    let cx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let props = (*props).clone();
    use_title(format!("Search: {} | ANiNFO", &(props.q)));

    let result: UseFutureHandle<Result<QueryResult, gloo::net::Error>> =
        {
            let cx = cx.clone();
            let props = props.clone();
            use_future_with(
                (
                    (*cx).nsfw,
                    (*cx).query.clone(),
                    (*cx).cur_page,
                    (*cx).language.clone(),
                ),
                |_| async move {
                    let (has_changed, _) = (*cx).has_changed();

                    if has_changed {
                        let result = force_req::<QueryResult>(&format!(
                            "https://api.jikan.moe/v4/anime?q={}&page={}{}",
                            (props.q).clone(),
                            &(props.page),
                            handle_nsfw(&cx)
                        ))
                        .await;
                        if let Ok(r) = result {
                            cx.dispatch((*cx).update_cache_into(
                                (*cx).cache.update_search_result(Some(r.clone())),
                            ));
                            return Ok(r);
                        } else if let Err(e) = result {
                            log!(format!("{:?}", e));
                            return Err(e);
                        } else {
                            return Err(GlooError(format!("Unknown error.")));
                        }
                    } else {
                        if let Some(r) = (*cx).cache.search_result.clone() {
                            return Ok(r);
                        } else {
                            let result = force_req::<QueryResult>(&format!(
                                "https://api.jikan.moe/v4/anime?q={}&page={}{}",
                                (props.q).clone(),
                                &(props.page),
                                handle_nsfw(&cx)
                            ))
                            .await;
                            if let Ok(r) = result {
                                cx.dispatch((*cx).update_cache_into(
                                    (*cx).cache.update_search_result(Some(r.clone())),
                                ));
                                return Ok(r);
                            } else if let Err(e) = result {
                                log!(format!("{:?}", e));
                                return Err(e);
                            } else {
                                return Err(GlooError(format!("Unknown error.")));
                            }
                        }
                    }
                    // fetch_data_into::<QueryResult>(&format!("https://api.jikan.moe/v4/anime?q={}&page={}", (props.q).clone(), &(props.page))).await
                },
            )?
        };

    let html_result = match (*result).as_ref() {
        Ok(v) => {
            if v.data.len() == 0 {
                html! {
                    <div class="search-result-wrapper">
                    <div class="written-content">
                        <div class="content-header">
                            <span><h2 class="content-ttl vc-text">{"Oops!"}</h2></span>
                        </div>
                        <p style="text-align: center !important;">{"No anime matching your search query is found."}</p>
                    </div>
                    </div>
                }
            } else {
                html! {
                    <div class="search-result-wrapper">
                        <div class="content-header">
                            <span><h2 class="content-ttl vc-text">{"Search Result"}</h2></span>
                        </div>
                        {
                            if v.pagination.last_visible_page > 1 {
                                html!{
                                    <div class="page-btn-wrapper">
                                    {into_page_btns(cx.clone(), (*v).pagination.clone(), nav.clone(), (props.q).clone(), props.page)}
                                    </div>
                                }
                            } else {
                                html!{}
                            }
                        }

                        <div class="cards-wrapper">
                            {into_anime_cards(&(v.data))}
                        </div>

                        {
                            if v.pagination.last_visible_page > 1 {
                                html!{
                                    <div class="page-btn-wrapper">
                                    {into_page_btns(cx.clone(), (*v).pagination.clone(), nav.clone(), (props.q).clone(), props.page)}
                                    </div>
                                }
                            } else {
                                html!{}
                            }
                        }
                    </div>
                }
            }
        }
        Err(e) => {
            match e {
                JsError(_) => {
                    log!("js err");
                    // cx.dispatch((*cx).update_hash_into(0));
                    nav.push(&Route::ErrorPage {
                        app_err: AppErr::serde_default(),
                    });
                    let nav = nav.clone();
                    gloo::timers::callback::Timeout::new(3000, move || {
                        nav.back();
                    })
                    .forget();
                    html! {<ErrorPage app_err={AppErr::js_default()}/>}
                }
                SerdeError(_) => {
                    log!("serde err");
                    // cx.dispatch((*cx).update_hash_into(0));
                    nav.push(&Route::ErrorPage {
                        app_err: AppErr::serde_default(),
                    });
                    let nav = nav.clone();
                    gloo::timers::callback::Timeout::new(3000, move || {
                        nav.back();
                    })
                    .forget();
                    html! {<ErrorPage app_err={AppErr::serde_default()}/>}
                }
                GlooError(_) => {
                    log!("gloo err");
                    // cx.dispatch((*cx).update_hash_into(0));
                    nav.push(&Route::ErrorPage {
                        app_err: AppErr::serde_default(),
                    });
                    let nav = nav.clone();
                    gloo::timers::callback::Timeout::new(3000, move || {
                        nav.back();
                    })
                    .forget();
                    html! {<ErrorPage app_err={AppErr::gloo_default()}/>}
                }
            }
        }
    };

    Ok(html!({ html_result }))
}

#[function_component(SearchResult)]
pub fn search_result(props: &Props) -> Html {
    {
        use_effect(|| {
            // log!("page_btn_hook fired.");
            wasm_bindgen_futures::spawn_local(async {
                while let None = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("page-btn-selected")
                {
                    // log!("while loop.");
                    gloo::timers::future::TimeoutFuture::new(100).await;
                }
                let selected_btn = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("page-btn-selected")
                    .unwrap();
                let dom_rect = selected_btn
                    .get_bounding_client_rect()
                    .dyn_into::<DomRectReadOnly>()
                    .unwrap();
                let left = dom_rect.left();
                let vw = web_sys::window()
                    .unwrap()
                    .inner_width()
                    .unwrap()
                    .as_f64()
                    .unwrap();
                log!(format!("left: {}", left));
                log!(format!("vw: {}", vw));

                while web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_class_name("page-btn-wrapper")
                    .length()
                    == 0
                {
                    gloo::timers::future::TimeoutFuture::new(100).await;
                    // log!("while loop.");
                }

                if vw <= 1000f64 {
                    if left > vw {
                        let btn_wrappers: HtmlCollection = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_elements_by_class_name("page-btn-wrapper");
                        (0..btn_wrappers.length()).into_iter().for_each(|n| {
                            let wrapper = btn_wrappers.get_with_index(n).unwrap();
                            wrapper.scroll_by_with_x_and_y(left - (vw / 2f64) + 20f64, 0f64);
                        })
                    }
                } else {
                    if left > (vw / 2f64) + 400f64 as f64 {
                        let btn_wrappers: HtmlCollection = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_elements_by_class_name("page-btn-wrapper");
                        (0..btn_wrappers.length()).into_iter().for_each(|n| {
                            let wrapper = btn_wrappers.get_with_index(n).unwrap();
                            wrapper.scroll_by_with_x_and_y(left - (vw / 2f64) + 20f64, 0f64);
                        })
                    }
                }
            });
        });
    }

    html! {
        <Suspense fallback={html!(<Loading/>)}>
            <Content q={(props.q).clone()} page={props.page}/>
        </Suspense>
    }
}
