use web_sys::HtmlInputElement;

use crate::prelude::*;

#[function_component(Content)]
fn content() -> HtmlResult {
    let (cx, nav) = (
        use_context::<AppContext>().unwrap(),
        use_navigator().unwrap(),
    );
    let q_filter = use_store::<QueryFilter>();
    let q_sort = use_store::<QuerySort>();
    let link_state = use_state(|| "".to_string());
    let cur_page = use_store::<ExplorePage>();

    let result: UseFutureHandle<Result<QueryResult, gloo::net::Error>> = {
        let cx = cx.clone();
        let qf = q_filter.0.clone();
        let qs = q_sort.0.clone();
        let link_state = link_state.clone();
        let cur_page = cur_page.clone();

        use_future_with(
            ((*cx).nsfw, qf.clone(), *qs, cur_page.0.clone()),
            |_| async move {
                let link = format!(
                    "https://api.jikan.moe/v4/anime?sfw={}&page={}{}{}&",
                    !(*cx).nsfw,
                    cur_page.0 .0,
                    (*qf).to_params(),
                    (*qs).to_params()
                );
                link_state.set((&link).to_string());
                force_req::<QueryResult>(&link).await
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
                let cx = cx.clone();
                let link_state = link_state.clone();
                let theme = handle_theme(&cx);
                let last_page = v.pagination.last_visible_page;
                let cur_page = cur_page.clone();
                html! {
                    <div class="search-result-wrapper" onload={|_| {log!("result-wrapper loaded.")}}>
                        <div class="content-header">
                            <span><h2 class="content-ttl vc-text">{"Exploration Result"}</h2></span>
                        </div>

                        <div class="page-btn-wrapper">
                        {
                            html! {
                                {
                                    (1..last_page+1).into_iter().map(|n| {
                                        let (cur_page_s, cur_page_disp) = cur_page.clone();
                                        if n as u32 == (*(cur_page_s)).0 {
                                            html! {
                                                <button id="page-btn-selected" class={format!("cursor-pointer page-btn selected {}", &theme)} onclick={move |_: MouseEvent| {
                                                    cur_page_disp.set(ExplorePage(n as u32));
                                                    web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                                                }}>{n}</button>
                                            }
                                        } else {
                                            html! {
                                                <button class={format!("cursor-pointer page-btn {}", theme)} onclick={move |_: MouseEvent| {
                                                    cur_page_disp.set(ExplorePage(n as u32));
                                                    web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                                                }}>{n}</button>
                                            }
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        }
                        </div>

                        <div class="cards-wrapper">
                            {into_anime_cards(&(v.data))}
                        </div>

                        <div class="page-btn-wrapper">
                        {
                            html! {
                                {
                                    (1..last_page+1).into_iter().map(|n| {
                                        let (cur_page_s, cur_page_disp) = cur_page.clone();
                                        if n as u32 == cur_page_s.0 {
                                            html! {
                                                <button id="page-btn-selected" class={format!("cursor-pointer page-btn selected {}", &theme)} onclick={move |_: MouseEvent| {
                                                    log!("btn pushed");
                                                    cur_page_disp.set(ExplorePage(n as u32));
                                                    web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                                                }}>{n}</button>
                                            }
                                        } else {
                                            html! {
                                                <button class={format!("cursor-pointer page-btn {}", theme)} onclick={move |_: MouseEvent| {
                                                    log!("btn pushed");
                                                    cur_page_disp.set(ExplorePage(n as u32));
                                                    web_sys::window().expect("Missing window.").scroll_to_with_x_and_y(0f64, 0f64);
                                                }}>{n}</button>
                                            }
                                        }
                                    }).collect::<Html>()
                                }
                            }
                        }
                        </div>

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

#[function_component(ExploreAnime)]
pub fn explore_anime() -> Html {
    let qs = use_store::<QuerySort>();
    let qf = use_store::<QueryFilter>();
    let cx = use_context::<AppContext>().unwrap();
    let cur_page = use_store::<ExplorePage>();
    use_title("Explore Anime | ANiNFO".to_string());

    let reset_genre = {
        let qf = qf.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            qf.1.set(qf.0.reset_genres());
        })
    };

    let reset_genre_excl = {
        let qf = qf.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            qf.1.set(qf.0.reset_genres_exclude());
        })
    };

    let start_year_onselect = {
        let qf = qf.clone();
        Callback::from(move |i: InputEvent| {
            let elem = i.target_unchecked_into::<HtmlInputElement>();
            log!("Startyear set to: ", &elem.value());
            if &elem.value() == "N/A" {
                qf.1.set((*(qf.0)).remove_start_year());
            } else {
                qf.1.set((*(qf.0)).add_start_year(&elem.value()));
            }
        })
    };

    let end_year_onselect = {
        let qf = qf.clone();
        Callback::from(move |i: InputEvent| {
            let elem = i.target_unchecked_into::<HtmlInputElement>();
            log!("Endyear set to: ", &elem.value());
            if &elem.value() == "N/A" {
                qf.1.set((*(qf.0)).remove_end_year());
            } else {
                qf.1.set((*(qf.0)).add_end_year(&elem.value()));
            }
        })
    };

    let reset_year = {
        let qf = qf.clone();
        Callback::from(move |_: MouseEvent| qf.1.set(qf.0.remove_start_year().remove_end_year()))
    };

    html! {
        <>
        <section class={format!("explore-filter-wrapper max-1600px-resp {}", &handle_theme(&cx))}>
            <h2 class="filter-header">{"Anime Filters"}</h2>
            <div id="genre-include-header" class={format!("explore-filter-header obj-level-1 hover-highlight {}", &handle_theme(&cx))} onclick={
                {
                    let cx = cx.clone();
                    move |_| {
                    let elem = get_elem_by_id!("genre-include");
                    let header = web_sys::window().unwrap().document().unwrap().get_element_by_id("genre-include-header").unwrap();
                    let icon = web_sys::window().unwrap().document().unwrap().get_element_by_id("icon-down-genre").unwrap();
                    let cur_class = elem.get_attribute("class").unwrap();
                    if cur_class.contains("active") {
                        elem.set_attribute("class", &format!("explore-filters {}", &handle_theme(&cx)));
                        icon.set_attribute("class", &format!("header-icon icon-{}", &handle_theme(&cx)));
                        header.set_attribute("class", &format!("explore-filter-header obj-level-1 hover-highlight {}", &handle_theme(&cx)));
                    } else {
                        elem.set_attribute("class", &format!("explore-filters-active {}", &handle_theme(&cx)));
                        header.set_attribute("class", &format!("explore-filter-header-active obj-level-1 hover-highlight {}", &handle_theme(&cx)));
                        icon.set_attribute("class", &format!("header-icon-active icon-{}", &handle_theme(&cx)));
                    }
                }}
            }>
            <a class="a-btn ripple {}">
                <img id="icon-down-genre" class={format!("header-icon icon-{}", &handle_theme(&cx))} src="./static/down.png" />
            </a>
            {
                if ((*(qf.0)).genres).len() == 0 {
                    html!{
                        <h3 class="disp-flex-normal">{"Genres"}</h3>
                    }
                } else {
                    html!{
                        <>
                        <h3 class="disp-flex"><span>{"Genres"}
                        <span class="header-small">
                        {": "}
                        {
                            ((*(qf.0)).genres).iter().enumerate().map(|(i, g)| {
                                if i+1 == ((*(qf.0)).genres).len() {
                                    html!{{&g.name}}
                                } else {
                                    html!{<>{&g.name}{", "}</>}
                                }
                            })
                            .collect::<Html>()}
                            </span></span><a class="header-small u_onhover cursor-pointer" onclick={reset_genre}>{"Reset"}</a>
                        </h3>
                        </>}
                }
            }
            </div>


            <section id="genre-include" class={format!("explore-filters {}", &handle_theme(&cx))}>
                {mal_genres().into_iter().map(|g| {
                    let qf = qf.clone();
                    let cur_page = cur_page.clone();
                    let onclick = {
                        let g = g.clone();
                        let qf = qf.clone();
                        let cx = cx.clone();
                        let cur_page = cur_page.clone();
                        Callback::from(move |_: MouseEvent| {
                            let v = (*(qf.0)).clone();
                            cur_page.1.set(ExplorePage(1));
                            if v.genres.contains(&g) {
                                qf.1.set(v.remove_genres(&g));
                                let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("genre-btn-{}", g.mal_id)).unwrap();
                                btn.set_attribute("class", &format!("genre-btn hover-highlight {}", handle_theme(&cx)));
                            } else {
                                qf.1.set(v.add_genres(&g));
                                let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("genre-btn-{}", g.mal_id)).unwrap();
                                btn.set_attribute("class", &format!("genre-btn-active hover-highlight {}", handle_theme(&cx)));
                            }
                        })
                    };
                    if &g.name == "Hentai" {
                        if cx.nsfw {
                            html!{{
                                if (qf.0).genres.contains(&g) {
                                    html!{<button id={format!("genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn-active hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                                } else {
                                    html!{<button id={format!("genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                                }
                            }

                            }
                        } else {
                            html!{}
                        }
                    } else {
                        html!{{
                            if (qf.0).genres.contains(&g) {
                                html!{<button id={format!("genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn-active hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                            } else {
                                html!{<button id={format!("genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                            }
                        }
                        }
                    }
                }).collect::<Html>()}
            </section>


            <div id="genre-exclude-header" class={format!("explore-filter-header obj-level-1 hover-highlight {}", &handle_theme(&cx))} onclick={
                {
                    let cx = cx.clone();

                    move |_| {
                    let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("genre-exclude").unwrap();
                    let header = web_sys::window().unwrap().document().unwrap().get_element_by_id("genre-exclude-header").unwrap();
                    let icon = web_sys::window().unwrap().document().unwrap().get_element_by_id("icon-down-genre-exclude").unwrap();
                    let cur_class = elem.get_attribute("class").unwrap();
                    if cur_class.contains("active") {
                        elem.set_attribute("class", &format!("explore-filters {}", &handle_theme(&cx)));
                        icon.set_attribute("class", &format!("header-icon icon-{}", &handle_theme(&cx)));
                        header.set_attribute("class", &format!("explore-filter-header obj-level-1 hover-highlight {}", &handle_theme(&cx)));

                    } else {
                        icon.set_attribute("class", &format!("header-icon-active icon-{}", &handle_theme(&cx)));
                        elem.set_attribute("class", &format!("explore-filters-active {}", &handle_theme(&cx)));
                        header.set_attribute("class", &format!("explore-filter-header-active obj-level-1 hover-highlight {}", &handle_theme(&cx)));

                    }
                }}
            }>
            <a class="a-btn ripple {}">
                <img id="icon-down-genre-exclude" class={format!("header-icon icon-{}", &handle_theme(&cx))} src="./static/down.png" />
            </a>
            {
                if ((*(qf.0)).genres_exclude).len() == 0 {
                    html!{
                        <h3 class="disp-flex-normal">

                        {"Genres to Exclude"}</h3>
                    }
                } else {
                    html!{
                        <>
                        <h3 class="disp-flex">
                        <span>
                        {"Genres to Exclude"}
                        <span class="header-small">
                        {": "}
                        {
                            ((*(qf.0)).genres_exclude).iter().enumerate().map(|(i, g)| {
                                if i+1 == ((*(qf.0)).genres_exclude).len() {
                                    html!{{&g.name}}
                                } else {
                                    html!{<>{&g.name}{", "}</>}
                                }
                            })
                            .collect::<Html>()}
                            </span></span><a class="header-small u_onhover cursor-pointer" onclick={reset_genre_excl}>{"Reset"}</a>
                        </h3>
                        </>}
                }
            }
            </div>

            <section id="genre-exclude" class={format!("explore-filters {}", &handle_theme(&cx))}>
                {mal_genres().into_iter().map(|g| {
                    let qf = qf.clone();
                    let cur_page = cur_page.clone();
                    let onclick = {
                        let g = g.clone();
                        let qf = qf.clone();
                        let cx = cx.clone();
                        let cur_page = cur_page.clone();
                        Callback::from(move |_: MouseEvent| {
                            let v = (*(qf.0)).clone();
                            cur_page.1.set(ExplorePage(1));
                            if v.genres_exclude.contains(&g) {
                                qf.1.set(v.remove_genres_exclude(&g));
                                let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("excl-genre-btn-{}", g.mal_id)).unwrap();
                                btn.set_attribute("class", &format!("genre-btn hover-highlight {}", handle_theme(&cx)));
                            } else {
                                qf.1.set(v.add_genres_exclude(&g));
                                let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("excl-genre-btn-{}", g.mal_id)).unwrap();
                                btn.set_attribute("class", &format!("genre-btn-active hover-highlight {}", handle_theme(&cx)));
                            }
                        })
                    };
                    if &g.name == "Hentai" {
                        if cx.nsfw {
                            html!{{
                                if (qf.0).genres_exclude.contains(&g) {
                                    html!{<button id={format!("excl-genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn-active hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                                } else {
                                    html!{<button id={format!("excl-genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                                }
                            }

                            }
                        } else {
                            html!{}
                        }
                    } else {
                        html!{{
                            if (qf.0).genres_exclude.contains(&g) {
                                html!{<button id={format!("excl-genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn-active hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                            } else {
                                html!{<button id={format!("excl-genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
                            }
                        }
                        }
                    }
                }).collect::<Html>()}
            </section>

            <div id="year-header" class={format!("explore-filter-header obj-level-1 hover-highlight {}", &handle_theme(&cx))} onclick={
                {
                    let cx = cx.clone();
                    move |_| {
                    let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id("year-filter").unwrap();
                    let header = web_sys::window().unwrap().document().unwrap().get_element_by_id("year-header").unwrap();
                    let icon = web_sys::window().unwrap().document().unwrap().get_element_by_id("icon-down-year").unwrap();
                    let cur_class = elem.get_attribute("class").unwrap();
                    if cur_class.contains("active") {
                        elem.set_attribute("class", &format!("explore-filters {}", &handle_theme(&cx)));
                        icon.set_attribute("class", &format!("header-icon icon-{}", &handle_theme(&cx)));
                        header.set_attribute("class", &format!("explore-filter-header obj-level-1 hover-highlight {}", &handle_theme(&cx)));
                    } else {
                        elem.set_attribute("class", &format!("explore-filters-active {}", &handle_theme(&cx)));
                        header.set_attribute("class", &format!("explore-filter-header-active obj-level-1 hover-highlight {}", &handle_theme(&cx)));
                        icon.set_attribute("class", &format!("header-icon-active icon-{}", &handle_theme(&cx)));
                    }
                }}
            }>
            <a class="a-btn ripple {}">
            <img id="icon-down-year" class={format!("header-icon icon-{}", &handle_theme(&cx))} src="./static/down.png"/>
            </a>
            {
                // if let  None = (*(qf.0)).start_date.as_ref() {
                //     html!{
                //         <h3>{"Years"}</h3>
                //     }
                // } else {
                //     html!{
                //         <>
                //         <h3 class="disp-flex"><span>{"Years"}
                //         <span class="header-small">
                //         {": "}
                //         {
                //             &(*(qf.0)).start_date.as_ref().unwrap()[..4]
                //         }{" - "}
                //         {
                //             if let  None = (*(qf.0)).end_date.as_ref() {
                //                 html!{}
                //             } else {
                //                 html!{
                //                     <>
                //                     {
                //                         &(*(qf.0)).end_date.as_ref().unwrap()[..4]
                //                     }
                //                     </>
                //                 }
                //             }

                //         }
                //             </span></span><a class="header-small u_onhover cursor-pointer">{"Reset"}</a>
                //         </h3>
                //         </>}
                // }

                match ((*(qf.0)).start_date.as_ref(), (*(qf.0)).end_date.as_ref()) {
                    (None, None) => html!{<h3>{"Years"}</h3>},
                    (Some(ref y), None) => {
                        html!{
                            <>
                            <h3 class="disp-flex"><span>{"Years"}
                                <span class="header-small">
                                {": "}{&y[..4]}{" - "}
                                </span></span><a class="header-small u_onhover cursor-pointer" onclick={reset_year}>{"Reset"}</a>
                            </h3>
                            </>
                        }
                    },
                    (Some(ref y1), Some(ref y2)) => {
                        html!{
                            <>
                            <h3 class="disp-flex"><span>{"Years"}
                                <span class="header-small">
                                {
                                    if y1[..4] == y2[..4] {
                                    html!{<>{": "}{&y1[..4]}</>}
                                    } else {
                                        html!{<>{": "}{&y1[..4]}{" - "}{&y2[..4]}</>}
                                    }
                                }
                                </span></span><a class="header-small u_onhover cursor-pointer" onclick={reset_year}>{"Reset"}</a>
                            </h3>
                            </>
                        }
                    },
                    (None, Some(ref y)) => {
                        html!{
                            <>
                            <h3 class="disp-flex"><span>{"Years"}
                                <span class="header-small">
                                {": "}{"- "}{&y[..4]}
                                </span></span><a class="header-small u_onhover cursor-pointer" onclick={reset_year}>{"Reset"}</a>
                            </h3>
                            </>
                        }
                    }
                }
            }
            </div>

            <section id="year-filter" class={format!("explore-filters {}", &handle_theme(&cx))} >
                <div>
                    <label for="start-year">{"Year start "}</label>
                    <select id="start-year" oninput={start_year_onselect}>
                    {
                        match ( (*(qf.0)).start_date.as_ref(), (*(qf.0)).end_date.as_ref() ) {
                            (None, None) => {
                                html!{
                                            <>
                                            <option selected={true}>{"N/A"}</option>
                                            {
                                                (1900..cur_year()+1).rev().map(|n| {
                                                    html!{<option value={format!("{}", n)}>{n}</option>}
                                                }).collect::<Html>()
                                            }
                                            </>
                                        }
                            },
                            (Some(ref y_start), None) => {
                                let y_start = y_start[..4].parse::<u32>().unwrap();
                                html!{
                                    <>
                                    <option selected={true}>{"N/A"}</option>
                                    {
                                        (1900..cur_year()+1).rev().map(|n| {
                                            if n == y_start {
                                                html!{<option value={format!("{}", n)} selected={true}>{n}</option>}
                                            } else {
                                                html!{<option value={format!("{}", n)}>{n}</option>}
                                            }
                                        }).collect::<Html>()
                                    }
                                    </>
                                }

                            },
                            (Some(ref y_start), Some(ref y_end)) => {
                                let y_start = y_start[..4].parse::<u32>().expect("Year must be valid.");
                                let y_end = y_end[..4].parse::<u32>().expect("Year must be valid.");
                                (1900..y_end+1).rev().map(|n| {
                                    if n == y_start {
                                        html!{<option value={format!("{}", n)} selected={true}>{n}</option>}
                                    } else {
                                        html!{<option value={format!("{}", n)}>{n}</option>}
                                    }
                                }).collect::<Html>()

                            },
                            (None, Some(ref y_end)) => {
                                let y_end = y_end[..4].parse::<u32>().expect("Year must be valid.");
                                html!{
                                    <>
                                    <option selected={true}>{"N/A"}</option>
                                    {
                                        (y_end..cur_year()+1).rev().map(|n| {
                                            html!{<option value={format!("{}", n)}>{n}</option>}
                                        }).collect::<Html>()
                                    }
                                    </>
                                }
                            }
                        }
                    }


                    </select>
                </div>

                <div>
                    <label for="end-year">{"Year end "}</label>
                    <select id="end-year" oninput={end_year_onselect}>
                    {
                        match ( (*(qf.0)).start_date.as_ref(), (*(qf.0)).end_date.as_ref() ) {
                            (None, None) => {
                                html!{
                                            <>
                                            <option selected={true}>{"N/A"}</option>
                                            {
                                                (1900..cur_year()+1).rev().map(|n| {
                                                    html!{<option value={format!("{}", n)}>{n}</option>}
                                                }).collect::<Html>()
                                            }
                                            </>
                                        }
                            },
                            (Some(ref y_start), None) => {
                                let y_start = y_start[..4].parse::<u32>().unwrap();
                                html!{
                                    <>
                                    <option selected={true}>{"N/A"}</option>
                                    {
                                        (y_start..cur_year()+1).rev().map(|n| {
                                            html!{<option value={format!("{}", n)}>{n}</option>}
                                        }).collect::<Html>()
                                    }
                                    </>
                                }

                            },
                            (Some(ref y_start), Some(ref y_end)) => {
                                let y_start = y_start[..4].parse::<u32>().expect("Year must be valid.");
                                let y_end = y_end[..4].parse::<u32>().expect("Year must be valid.");
                                (y_start..cur_year()+1).rev().map(|n| {
                                    if n == y_end {
                                        html!{<option value={format!("{}", n)} selected={true}>{n}</option>}
                                    } else {
                                        html!{<option value={format!("{}", n)}>{n}</option>}
                                    }
                                }).collect::<Html>()

                            },
                            (None, Some(ref y_end)) => {
                                let y_end = y_end[..4].parse::<u32>().expect("Year must be valid.");
                                (1900..cur_year()+1).rev().map(|n| {
                                    if n == y_end {
                                        html!{<option value={format!("{}", n)} selected={true}>{n}</option>}
                                    } else {
                                        html!{<option value={format!("{}", n)}>{n}</option>}
                                    }
                                }).collect::<Html>()
                            }
                        }
                        // if let Some(ref y) =  {
                        //     let y = y[..4].parse::<u32>().unwrap();
                        //     if let Some(ref y_start) = (*(qf.0)).start_date.as_ref() {
                        //
                        //         (y_start..cur_year()+1).rev().map(|n| {
                        //             if n == y {
                        //                 html!{<option value={format!("{}", n)} selected={true}>{n}</option>}
                        //             } else {
                        //                 html!{<option value={format!("{}", n)}>{n}</option>}
                        //             }
                        //         }).collect::<Html>()
                        //     } else {
                        //         (1900..cur_year()+1).rev().map(|n| {
                        //             if n == y {
                        //                 html!{<option value={format!("{}", n)} selected={true}>{n}</option>}
                        //             } else {
                        //                 html!{<option value={format!("{}", n)}>{n}</option>}
                        //             }
                        //         }).collect::<Html>()
                        //     }
                        // } else {
                        //     html!{
                        //         <>
                        //         <option selected={true}>{"N/A"}</option>
                        //         {
                        //             (1900..cur_year()+1).rev().map(|n| {
                        //                 html!{<option value={format!("{}", n)}>{n}</option>}
                        //             }).collect::<Html>()
                        //         }
                        //         </>
                        //     }
                        // }
                    }


                    </select>
                </div>
            </section>

            // <h3>{"Year"}</h3>

            // <section id="year-filter" class="explore-filters">
            //     {mal_genres().into_iter().map(|g| {
            //         let qf = qf.clone();
            //         let onclick = {
            //             let g = g.clone();
            //             let qf = qf.clone();
            //             let cx = cx.clone();
            //             Callback::from(move |_: MouseEvent| {
            //                 let v = (*(qf.0)).clone();
            //                 if v.genres_exclude.contains(&g) {
            //                     qf.1.set(v.remove_genres_exclude(&g));
            //                     let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("excl-genre-btn-{}", g.mal_id)).unwrap();
            //                     btn.set_attribute("class", &format!("genre-btn hover-highlight {}", handle_theme(&cx)));
            //                 } else {
            //                     qf.1.set(v.add_genres_exclude(&g));
            //                     let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("excl-genre-btn-{}", g.mal_id)).unwrap();
            //                     btn.set_attribute("class", &format!("genre-btn-active hover-highlight {}", handle_theme(&cx)));
            //                 }
            //             })
            //         };
            //         html!{{
            //             if (qf.0).genres_exclude.contains(&g) {
            //                 html!{<button id={format!("excl-genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn-active hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
            //             } else {
            //                 html!{<button id={format!("excl-genre-btn-{}", g.mal_id)} {onclick} class={format!("genre-btn hover-highlight {}", &handle_theme(&cx))}>{&g.name}</button>}
            //             }
            //         }

            //         }
            //     }).collect::<Html>()}
            // </section>

            <section class="Result Orderings">

            </section>
        </section>
        <Suspense fallback={html!(<Loading/>)}>
            <Content/>
        </Suspense>
        </>
    }
}
