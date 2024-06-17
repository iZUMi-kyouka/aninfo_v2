use crate::prelude::*;

pub fn into_anime_cards(v: &Vec<AnimeObj>) -> Html {
    v.into_iter()
        .map(|anime_obj| html! (<AnimeCard anime_obj={anime_obj.clone()} />))
        .collect::<Html>()
}

pub fn into_anime_cards_minimal(v: &Vec<AnimeRecWrapper>) -> Html {
    if v.is_empty() {
        return html! {<span class="no-result">{"There are no recommended anime."}</span>};
    }
    html! {
        <div id="ani-rec-card-wrapper" class="ani-rec-card-wrapper">
        {v.into_iter().map(|anime_obj| {
            let anime_obj = &anime_obj.entry;
            html! (<AnimeCardMinimal ao={anime_obj.clone()} />)
        }).collect::<Html>()}
        </div>
    }
}

pub fn into_char_cards(v: &Vec<Char>) -> Html {
    if v.is_empty() {
        "Character data is currently unavailable.".into()
    } else {
        v.into_iter()
            .map(|char| html! (<CharacterCard char={char.clone()}/>))
            .collect::<Html>()
    }
}

pub fn into_eps_cards(cx: &AppContext, v: &Vec<AnimeEpisode>, ao: AnimeObj, full: bool) -> Html {
    if v.is_empty() {
        return "Episodes data is currently unavailable.".into();
    }
    let mut m = 0;
    let theme = handle_theme(cx);

    v.into_iter().map(|eo| {
        m += 1;
        let cx = cx.clone();
        let ao = ao.clone();
        // let ttl_def = eo.title.clone();
        // let ttl_en = eo.title_romanji.clone().unwrap_or(ttl_def.clone());
        
        let open_dwld = Callback::from(move |_: MouseEvent| {
            let ao = ao.clone();
            let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("eps-{}", m)).unwrap();
            let cur_state = elem.get_attribute("class").unwrap();

            // let result: Arc<Mutex<Vec<Vec<String>>>> = Arc::new(Mutex::new(vec![]));
            // let ttl_def = ttl_def.clone();
            // let ttl_en = ttl_en.clone();
            
            // let result_cloned = Arc::clone(&result);
            
            wasm_bindgen_futures::spawn_local(async move {
                let ttl_def = ao.titles[0].title.clone();
                let ttl_en = ao.title_english.clone().unwrap();
                log!("getting torrent.");
                let result = get_torrents(&ttl_en, &ttl_def, m, &[], full).await;
                log!(format!("{:#?}", &result));
            });


            if cur_state == "eps-dwld" {
                elem.set_attribute("class", "eps-dwld-active").expect("Cannot find class eps-dwld-active.");
            } else {
                elem.set_attribute("class", "eps-dwld").expect("Cannot find class: eps-dwld.");
            }
        });

        html! {
            <div class={format!("eps-card hover-highlight {}", theme)}>
                <div class="eps-header">
                    <div class="eps-info">
                        <div class="eps-no">{m}</div>
                        {if let Some(ref url) = eo.url {
                            html!{<span class="eps-title cursor-arrow"><a href={(*url).clone()}>{handle_eps_language(&cx, eo)}</a></span>}
                        } else {
                            html!{<span class="eps-title cursor-arrow">{handle_eps_language(&cx, eo)}</span>}
                        }
                        }
                    </div>
                    <div class="dropdown-icon"><a class="a-btn ripple cursor-pointer " onclick={open_dwld}><img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/dropdown.png" height="28px"/></a>
                    </div>
            </div>
                <div id={format!("eps-{}", m)} class="eps-dwld">
                {"Torrents go here."}
                </div>
            </div>
        }
    }).collect::<Html>()
}

pub fn handle_studios(cx: &AppContext, s_vec: Vec<MALObj>, nav: Navigator) -> Html {
    let theme = handle_theme(cx);
    if s_vec.len() == 0 {
        html! {}
    } else {
        let n = s_vec.len();
        let mut m = 1;
        html! {
            <p class="studio"><img loading="lazy" class={format!("icon-{}", theme)} id="airing-status" src="./static/tv.svg"/>{
                s_vec.into_iter().map(|studio| {
                    let nav = nav.clone();
                    m += 1;
                    if m <= n {
                        return html! {
                            <><a class="u_onhover cursor-pointer" onclick={
                                let studio_name = studio.name.clone();
                                move |_| {
                                    nav.push(&Route::AnimeResultStd {
                                        content_title: format!("Top Anime by {}", studio_name.clone()),
                                        page: 1,
                                        url: StdResultType::Producer(studio.mal_id) })
                                }
                            }>{studio.name}</a>{" | "}</>
                        };
                    } else {
                        return html! {
                            <><a class="u_onhover cursor-pointer" onclick={
                                let studio_name = studio.name.clone();
                                move |_| {
                                    nav.push(&Route::AnimeResultStd {
                                        content_title: format!("Top Anime by {}", studio_name.clone()),
                                        page: 1,
                                        url: StdResultType::Producer(studio.mal_id) })
                                }
                            }>{studio.name}</a></>
                        };
                    }
                }).collect::<Html>()
            }</p>
        }
    }
}

pub fn handle_year(cx: &AppContext, year: Option<u32>) -> Html {
    let theme = handle_theme(cx);
    match year {
        None => html! {},
        Some(n) => html! {
            <p class="year">
                <img loading="lazy" class={format!("icon-{}", theme)} id="year" src="./static/year.svg"/>{n}
            </p>
        },
    }
}

pub fn into_page_btns(
    cx: UseReducerHandle<AppCtx>,
    v: PaginationObj,
    nav: Navigator,
    q: String,
    cur_page: u32,
) -> Html {
    let theme = handle_theme(&cx);
    let last_page = v.last_visible_page;

    if last_page == 1 {
        return html! {};
    }

    html! {
        {
            (1..v.last_visible_page+1).into_iter().map(|n| {
                let nav = nav.clone();
                let q = q.clone();
                let cx = cx.clone();

                if n as u32 == cur_page {
                    html! {
                        <button id="page-btn-selected" class={format!("cursor-pointer page-btn selected {}", theme)} onclick={move |_: MouseEvent| {
                            log!("btn pushed");
                            cx.dispatch((*cx).update_page_into(n as u8));
                            web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                            nav.push(&Route::SearchResult {q: q.clone(), page: n as u32})
                        }}>{n}</button>
                    }
                } else {
                    html! {
                        <button class={format!("cursor-pointer page-btn {}", theme)} onclick={move |_: MouseEvent| {
                            log!("btn pushed");
                            cx.dispatch((*cx).update_page_into(n as u8));
                            web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                            nav.push(&Route::SearchResult {q: q.clone(), page: n as u32})
                        }}>{n}</button>
                    }
                }
            }).collect::<Html>()
        }
    }
}

pub fn into_page_btns_std(
    cx: UseReducerHandle<AppCtx>,
    v: PaginationObj,
    nav: Navigator,
    url: StdResultType,
    cur_page: u32,
    title: &str,
) -> Html {
    let theme = handle_theme(&cx);
    let last_page = v.last_visible_page;
    let title = title.to_string();

    if last_page == 1 {
        return html! {};
    }

    html! {
        {
            (1..v.last_visible_page+1).into_iter().map(|n| {
                let nav = nav.clone();
                let cx = cx.clone();
                let title = title.clone();
                let url = url.clone();
                if n as u32 == cur_page {
                    html! {
                        <button id="page-btn-selected" class={format!("cursor-pointer page-btn selected {}", theme)} onclick={move |_: MouseEvent| {
                            log!("btn pushed");
                            cx.dispatch((*cx).update_page_into(n as u8));
                            web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                            nav.push(&Route::AnimeResultStd { content_title: title.clone(), page: n as u32, url: url.clone() } )
                        }}>{n}</button>
                    }
                } else {
                    html! {
                        <button class={format!("cursor-pointer page-btn {}", theme)} onclick={move |_: MouseEvent| {
                            log!("btn pushed");
                            cx.dispatch((*cx).update_page_into(n as u8));
                            web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                            nav.push(&Route::AnimeResultStd { content_title: title.clone(), page: n as u32, url: url.clone() } )
                        }}>{n}</button>
                    }
                }
            }).collect::<Html>()
        }
    }
}

pub fn into_comment_cards() -> Html {
    todo!()
}

// pub handle_studios_universal(studios: Vec<MALObj>, class: String) -> Html {
//     let n = studios.len();
//     let mut m = 1;
//     if n == 0 {
//         return html!{};
//     } else {
//         html! {
//             <{
//                 studios.into_iter().map(|studio| {
//                     if m == n {
//                         html! (<>{studio.name}</>)
//                     } else {
//                         m += 1;
//                         html! {<>{studio.name}{ " | "}</>}
//                     }
//                 }).collect::<Html>()
//             }
//             </p>
//         }
//     }
// }
