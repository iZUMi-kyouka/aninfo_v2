use std::ops::Deref;

use crate::prelude::*;
use gloo::dialogs::alert;
use web_sys::{HtmlDialogElement, HtmlInputElement, HtmlButtonElement};

pub const FILTER_TYPES: [Filter; 6] = [
    Filter::BDRip,
    Filter::HEVC,
    Filter::DDP,
    Filter::AMZN,
    Filter::FLAC,
    Filter::AllEpisodes
];

#[derive(Properties, PartialEq, Clone)]
pub struct EpisodeCardProps {
    pub anime_ttl_def: String,
    pub anime_ttl_en: String,
    pub eo: Vec<AnimeEpisode>,
    pub eps_total: usize,
    pub total_pages: usize,
    pub mal_id: u64,
}

#[function_component]
pub fn EpisodeCards(props: &EpisodeCardProps) -> Html {
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let theme_cheap = use_state(|| theme.clone());
    let ttl_def = props.anime_ttl_def.clone();
    let ttl_en = props.anime_ttl_en.clone();
    let eps_total = props.eps_total;
    let mal_id = props.mal_id;
    let get_torrent_full = use_state(|| false);
    let show_episode_titles = use_state(|| false);

    let client_query = use_state(|| String::new());
    let update_client_query = {
        let query = client_query.clone();
        Callback::from(move |e: InputEvent| {
            let text = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            query.set(text.value().to_lowercase());
        })
    };

    let ttl_cheap_def = use_state(|| props.anime_ttl_def.clone());
    let ttl_cheap_en = use_state(|| props.anime_ttl_en.clone());

    let filters: UseStateHandle<Vec<Filter>> = use_state(|| vec![]);
    let cur_page = use_state(|| 1usize);
    let eps_start = use_state(|| 1usize);
    let eps_end = use_state(|| 1usize);
    let torrent_data_state: UseStateHandle<Vec<Option<Vec<Torrent>>>> = use_state(|| {
        (0..eps_total)
            .into_iter()
            .map(|_| None)
            .collect::<Vec<Option<Vec<Torrent>>>>()
    });
    let eo_full = use_state(|| {
        let mut eo_full = props
            .eo
            .clone()
            .into_iter()
            .map(|eo| Some(eo))
            .collect::<Vec<Option<AnimeEpisode>>>();
        while eo_full.len() < (eps_total) {
            eo_full.push(None);
        }
        eo_full
    }); 

    if eps_total == 0 {
        return html!(
            <div class={format!("episodes-wrapper {}", theme.clone())}>
            <div class="ad-section-header">
            <img id="episodes-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/episode.png"/><h2 id="ad-section-header" class="content-ttl">{"Episodes"}</h2></div>
            <span class="no-result">{"Episodes data is unavailable."}</span>
            </div>
        );
    }

    // Use effect to update when anime is changed
    {
        let cur_page = cur_page.clone();
        let eps_start = eps_start.clone();
        let eps_end = eps_end.clone();
        let eo_full = eo_full.clone();
        let eps_total = props.eps_total;
        let eo = props.eo.clone();
        let torrent_data_state = torrent_data_state.clone();

        use_effect_with(props.clone(), move |_| {
            // log!("resetting values...");
            cur_page.set(1usize);
            eps_start.set(1usize);
            if eps_total <= 100 {
                eps_end.set(eps_total);
            } else {
                eps_end.set(100);
            }

            torrent_data_state.set(
                (0..eps_total)
                    .into_iter()
                    .map(|_| None)
                    .collect::<Vec<Option<Vec<Torrent>>>>(),
            );

            eo_full.set({
                let mut eo_full = eo
                    .clone()
                    .into_iter()
                    .map(|eo| Some(eo))
                    .collect::<Vec<Option<AnimeEpisode>>>();
                // log!("eo_full length: ", eo_full.len());
                // while eo_full.len() < eps_total {
                // eo_full.push(None);
                // log!("updating eo_full length to", eo_full.len());
                // }
                if eo_full.len() < eps_total {
                    eo_full.resize(eps_total, None);
                    // log!("updating eo_full length to", eo_full.len());
                }

                eo_full
            })
        })
    }

    {
        let torrent_data_state = torrent_data_state.clone();
        use_effect_with(torrent_data_state, |_| {
            // log!("state changed.");
        });
    }

    let show_episode_titles_cb = {
        let show_episode_titles = show_episode_titles.clone();
        Callback::from(move |e: MouseEvent| {
            let document = web_sys::window().unwrap().document().unwrap();
            if *show_episode_titles {
                show_episode_titles.set(false);
                let checkbox = document.get_element_by_id("show-eps-ttl-checkbox").unwrap().dyn_into::<HtmlInputElement>().unwrap();
                checkbox.set_checked(false);
                let episodes = document.get_elements_by_class_name("eps-title");
                (0..episodes.length()).for_each(|i| {
                    episodes.get_with_index(i).unwrap().set_attribute("class", "eps-title cursor-arrow normally-blurred");
                });
            } else {
                show_episode_titles.set(true);
                let checkbox = web_sys::window().unwrap().document().unwrap().get_element_by_id("show-eps-ttl-checkbox").unwrap().dyn_into::<HtmlInputElement>().unwrap();
                checkbox.set_checked(true);
                let episodes = document.get_elements_by_class_name("eps-title");
                (0..episodes.length()).for_each(|i| {
                    episodes.get_with_index(i).unwrap().set_attribute("class", "eps-title cursor-arrow");
                });
            }
        })
    };

    // let bdrip_filter = {
    //     let ttl_cheap_def = ttl_cheap_def.clone();
    //     let ttl_cheap_en = ttl_cheap_en.clone();
    //     let filters = filters.clone();
    //     let torrent_data_state = torrent_data_state.clone();
    //     Callback::from(move |e: MouseEvent| {
    //         let ttl_cheap_def = ttl_cheap_def.clone();
    //         let ttl_cheap_en = ttl_cheap_en.clone();
    //         let torrent_data_state = torrent_data_state.clone();

    //         if (*filters).contains(&Filter::BDRip) {
    //             filters.set((*filters).clone().into_iter().filter(|f| {
    //                 *f != Filter::BDRip
    //             }).collect::<Vec<Filter>>());
    //         } else {
    //             filters.set({
    //                 let mut v = (*filters).clone();
    //                 v.push(Filter::BDRip);
    //                 v
    //             });
    //         }

    //         wasm_bindgen_futures::spawn_local(async move {
    //             let torrents = get_torrents(&*ttl_cheap_en, &*ttl_cheap_def, n, &*filters).await;
    //             torrent_data_state.set(torrents);
    //         });
    //     })
    // };

    // let hevc_filter = {
    //     let ttl_cheap_def = ttl_cheap_def.clone();
    //     let ttl_cheap_en = ttl_cheap_en.clone();
    //     let filters = filters.clone();
    //     let torrent_data_state = torrent_data_state.clone();
    //     Callback::from(move |e: MouseEvent| {
    //         let ttl_cheap_def = ttl_cheap_def.clone();
    //         let ttl_cheap_en = ttl_cheap_en.clone();
    //         let torrent_data_state = torrent_data_state.clone();

    //         if (*filters).contains(&Filter::HEVC) {
    //             filters.set((*filters).clone().into_iter().filter(|f| {
    //                 *f != Filter::HEVC
    //             }).collect::<Vec<Filter>>());
    //         } else {
    //             filters.set({
    //                 let mut v = (*filters).clone();
    //                 v.push(Filter::HEVC);
    //                 v
    //             });
    //         }

    //         wasm_bindgen_futures::spawn_local(async move {
    //             let torrents = get_torrents(&*ttl_cheap_en, &*ttl_cheap_def, n, &*filters).await;
    //             torrent_data_state.set(torrents);
    //         });
    //     })
    // };
    //  HTML result
    html! {
            <div class={format!("episodes-wrapper {}", theme.clone())}>
                <div class="ad-section-header">
                    <img id="episodes-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/episode.png"/>
                    <h2 id="ad-section-header" class="content-ttl">{"Episodes"}</h2>
                </div>

                <div class="ad-section-top-options">
                    <input type="checkbox" id="show-eps-ttl-checkbox" onclick={show_episode_titles_cb}/>
                    <label for="show-eps-ttl-checkbox">{" Show ALL episode titles (you can hover onto the specific episode title to view it)"}</label>
                </div>

                <div class="eps-wrapper">
                // Handle iterating all the available episodes and torrent fetching logic
                {
                    (*eps_start..*eps_end+1).into_iter().map(|n| {

                        let ttl_def = ttl_def.clone();
                        let ttl_en = ttl_en.clone();
                        let theme = theme.clone();

                        // Inform users that episode data is unavailable/currently fetching

                        {
                            let theme = theme.clone();
                            let torrent_data_state = torrent_data_state.clone();
                            let theme_cloned = theme.clone();
                            let filters = filters.clone();
                            let get_torrent_full = get_torrent_full.clone();
                            let query = client_query.clone();


                            let open_dwld = Callback::from(move |_: MouseEvent| {
                                let theme_cloned = theme_cloned.clone();
                                let torrent_data_state = torrent_data_state.clone();
                                let get_torrent_full = get_torrent_full.clone();
                                let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("eps-{}", n)).unwrap();
                                let dialog = elem.clone().dyn_into::<HtmlDialogElement>().unwrap();
                                let cur_state = elem.get_attribute("class").unwrap();
                                let ttl_def = ttl_def.clone();
                                let ttl_en = ttl_en.clone();
                                let filters = filters.clone();
                                query.set("".to_string());

                                {
                                    if let None = (*torrent_data_state)[(n-1) as usize] {
                                        wasm_bindgen_futures::spawn_local(async move {
                                            let torrent_data_state = torrent_data_state.clone();
                                            let get_torrent_full = get_torrent_full.clone();
                                            let ttl_def = ttl_def.clone();
                                            let ttl_en = ttl_en.clone();
                                            let result = get_torrents(&ttl_en, &ttl_def, n as u16, &*filters, *get_torrent_full).await;
                                            // log!(format!("{:#?}", result));
                                            torrent_data_state.set({
                                                log!(format!("eps {} data is updated.", n));
                                                let mut v = (*torrent_data_state).clone();
                                                v[(n-1) as usize] = Some(result);
                                                v
                                            });
                                        });
                                    } else {
                                        ()
                                    }
                                }

                                if cur_state.contains("eps-dwld-active") {
                                    dialog.close();
                                    elem.set_attribute("class", &format!("eps-dwld {}", theme_cloned.clone())).unwrap();
                                } else {
                                    dialog.show();
                                    let query_input = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("client-torrent-custom-query-{}", n)).unwrap();
                                    let query_input = query_input.dyn_into::<HtmlInputElement>().unwrap();
                                    query.set(query_input.value());
                                    elem.set_attribute("class", &format!("eps-dwld-active {}", theme_cloned)).unwrap();
                                }
                            });

                            html!{
                            <div class={format!("eps-card hover-highlight {}", theme)}>
                                <div class="eps-header">
                                    <div class="eps-info">
                                        <div class="eps-no">{n}</div>
                                        {
                                            if None == (*eo_full)[n-1] {
                                                html!{
                                                    <div class="eps-title">{"Episode data is currently unavailable."}</div>
                                                }
                                            } else if ((n-1) as usize) < (*eo_full).len() {
                                                if let Some(ref url) = (*eo_full)[(n-1) as usize].as_ref().unwrap().url.clone() {
                                                    html!{<span class="eps-title cursor-arrow normally-blurred"><a target="_blank" href={(*eo_full)[(n-1) as usize].as_ref().unwrap().url.as_ref().unwrap().to_string()}>{handle_eps_language(&cx, (*eo_full)[(n-1) as usize].as_ref().unwrap())}</a></span>}
                                                } else {
                                                    html!{<span class="eps-title cursor-arrow normally-blurred">{handle_eps_language(&cx, (*eo_full)[(n-1) as usize].as_ref().unwrap())}</span>}
                                                }
                                            } else {
                                                html!{"Loading.."}
                                            }
                                        }
                                    </div>
                                    <div class="dropdown-icon">
                                            <a class="a-btn ripple cursor-pointer" onclick={open_dwld}>
                                                <img class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/external.png"/>
                                            </a>
                                    </div>

                                </div>
                            </div>
                        }}
                    }).collect::<Html>()
                }
                </div>

                // Handle episode buttons
                {
                    if props.total_pages > 1 {
                    html!{
                        <div class="eps-page-btns-wrapper">
                        {(1..props.total_pages+1).into_iter().map(|n| {
                            let eps_start = eps_start.clone();
                            let eps_end = eps_end.clone();
                            let eps_total = eps_total.clone();
                            let cur_page = cur_page.clone();
                            let eo_full = eo_full.clone();
                            let pages_total = props.total_pages;
                            if n == (*cur_page) {
                                html! {
                                    <button id="page-btn-selected" class={format!("cursor-pointer page-btn selected hover-highlight {}", theme)} onclick={move |_: MouseEvent| {
                                        log!("btn pushed");
                                        if n == pages_total {
                                            eps_start.set((n-1)*100 + 1);
                                            eps_end.set(eps_total);
                                        } else {
                                            eps_start.set((n-1)*100 + 1);
                                            eps_end.set((n-1)*100 + 100);
                                        }

                                        let eo_full = eo_full.clone();
                                        let cur_page = cur_page.clone();
                                        {
                                            let eo_full = eo_full.clone();
                                            wasm_bindgen_futures::spawn_local(async move {
                                                let link = format!("https://api.jikan.moe/v4/anime/21/episodes?page={}", n);
                                                let result = force_req::<AnimeEpisodeWrapper>(&link).await.unwrap().data;
                                                let mut cur_eo = (*eo_full).clone();
                                                let mut i = n*100;
                                                result.into_iter().for_each(|eo| {
                                                    cur_eo[i] = Some(eo);
                                                    i += 1;
                                                });

                                                eo_full.set(cur_eo);

                                            });
                                        }

                                        cur_page.set(n);
                                        log!("eps page: ", n);
                                    }}>{format!("{} - {}", (n-1)*100+1,{
                                        if n == pages_total {
                                            eps_total
                                        } else {
                                            n*100
                                        }
                                    })}</button>
                                }
                            } else {
                                html! {
                                    <button class={format!("cursor-pointer page-btn hover-highlight {}", theme)} onclick={move |_: MouseEvent| {
                                        log!("btn pushed");
                                        if n == pages_total {
                                            eps_start.set((n-1)*100 + 1);
                                            eps_end.set(eps_total);
                                        } else {
                                            eps_start.set((n-1)*100 + 1);
                                            eps_end.set((n-1)*100 + 100);
                                        }

                                        let eo_full = eo_full.clone();
                                        let cur_page = cur_page.clone();
                                        {
                                            let eo_full = eo_full.clone();
                                            wasm_bindgen_futures::spawn_local(async move {
                                                let link = format!("https://api.jikan.moe/v4/anime/{}/episodes?page={}", mal_id, n);
                                                let result = force_req::<AnimeEpisodeWrapper>(&link).await.unwrap().data;
                                                let mut cur_eo = (*eo_full).clone();
                                                let mut i = (n-1)*100;
                                                result.into_iter().for_each(|eo| {
                                                    cur_eo[i] = Some(eo);
                                                    i += 1;
                                                });

                                                eo_full.set(cur_eo);

                                            });
                                        }

                                        cur_page.set(n);
                                        log!("eps page: ", n);
                                    }}>{format!("{} - {}", (n-1)*100+1, {
                                        if n == pages_total {
                                            eps_total
                                        } else {
                                            n*100
                                        }
                                    })}</button>
                                }
                            }
                        }).collect::<Html>()}
                        </div>
                    }

                } else {
                    html!{}
                }
            }

                // Handle torrent popup
                <div class="eps-dwld-wrapper">
                {(1..eps_total+1).into_iter().map(|n| {
                    let theme = theme.clone();
                    let theme_cloned = theme.clone();
                    let filters = filters.clone();
                    
                    let filter_callbacks = FILTER_TYPES
                        .iter()
                        .map(|filter_type| {
                            let ttl_cheap_def = ttl_cheap_def.clone();
                            let ttl_cheap_en = ttl_cheap_en.clone();
                            let filters = filters.clone();
                            let torrent_data_state = torrent_data_state.clone();
                            let get_torrent_full = get_torrent_full.clone();
                            let theme = theme.clone();
                            
                            Callback::from(move |_: MouseEvent| {
                                let ttl_cheap_def = ttl_cheap_def.clone();
                                let ttl_cheap_en = ttl_cheap_en.clone();
                                let torrent_data_state = torrent_data_state.clone();
                                let get_torrent_full = get_torrent_full.clone();
                                let theme = theme.clone();
                                let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("{}-btn-{}", filter_type.as_str(), n)).unwrap();

                                    
                                // Reset data state for ALL episodes
                                {
                                    let torrent_data_state = torrent_data_state.clone();
                                    let v = (*torrent_data_state).clone().iter_mut().map(|_| None).collect::<Vec<Option<Vec<Torrent>>>>();
                                    torrent_data_state.set(v);
                                }
                                
                                // Set class to keep track of button state
                                match (btn.get_attribute("class").unwrap().contains("active"), *get_torrent_full) {
                                    (true, true) => {
                                        btn.set_attribute("class", &format!("eps-filter-btn eps-filter hover-highlight {}", &theme)).unwrap();
                                    },
                                    (false, true) => btn.set_attribute("class", &format!("eps-filter-btn eps-filter-active hover-highlight {}", &theme)).unwrap(),
                                    (_, false) => {
                                        btn.set_attribute("class", &format!("eps-filter-btn eps-filter cursor-disabled {}", &theme)).unwrap();
                                        alert("'Enhanced Result' MUST be turned on to use these filters.");
                                        return ();
                                    }
                                }
                                
                                // Get the selected filters
                                let filters = filters.clone();
                                log!(format!("{} filter clicked.", &filter_type.as_str()));
                                let mut filter_to_req = vec![];
                                
                                // Remove from the selected filter
                                if (*filters).contains(&filter_type) {
                                    filters.set({
                                        let v = (*filters).clone().into_iter().filter(|f| {
                                        *f != *filter_type
                                    }).collect::<Vec<Filter>>();
                                    filter_to_req = v.clone();
                                    v
                                });
                                } else {
                                    filters.set({
                                        let mut v = (*filters).clone();
                                        v.push(*filter_type);
                                        filter_to_req = v.clone();
                                        v
                                    });
                                }
    
                                wasm_bindgen_futures::spawn_local(async move {
                                    let get_torrent_full = get_torrent_full.clone();
                                    let torrents = get_torrents(&*ttl_cheap_en, &*ttl_cheap_def, n as u16, &filter_to_req, *get_torrent_full).await;
                                    log!(format!("{:#?}", &torrents));
                                    let mut cur_t = (*torrent_data_state).clone().iter_mut().map(|_| None).collect::<Vec<Option<Vec<Torrent>>>>();
                                    cur_t[n-1] = Some(torrents);
                                    torrent_data_state.set(cur_t);
                                });
                            })
                        })
                            .collect::<Vec<Callback<_>>>();

                    let bdrip_filter = {
                        let ttl_cheap_def = ttl_cheap_def.clone();
                        let ttl_cheap_en = ttl_cheap_en.clone();
                        let filters = filters.clone();
                        let torrent_data_state = torrent_data_state.clone();
                        let get_torrent_full = get_torrent_full.clone();
                        let theme = theme.clone();
                        Callback::from(move |_: MouseEvent| {
                            let ttl_cheap_def = ttl_cheap_def.clone();
                            let ttl_cheap_en = ttl_cheap_en.clone();
                            let torrent_data_state = torrent_data_state.clone();
                            let get_torrent_full = get_torrent_full.clone();
                            let theme = theme.clone();
                            let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("bd-btn-{}", n)).unwrap();

                            if btn.get_attribute("class").unwrap().contains("active") {
                                btn.set_attribute("class", &format!("eps-filter hover-highlight {}", &theme));
                            } else {
                                btn.set_attribute("class", &format!("eps-filter-active hover-highlight {}", &theme));
                            }


                            {
                                let torrent_data_state = torrent_data_state.clone();
                                let mut v = (*torrent_data_state).clone();
                                v[n-1] = None;
                                torrent_data_state.set(v);
                            }

                            let filters = filters.clone();
                            log!("bdrip filter clicked.");
                            let mut filter_to_req = vec![];

                            if (*filters).contains(&Filter::BDRip) {
                                filters.set({
                                    let v = (*filters).clone().into_iter().filter(|f| {
                                    *f != Filter::BDRip
                                }).collect::<Vec<Filter>>();
                                filter_to_req = v.clone();
                                v
                            });
                            } else {
                                filters.set({
                                    let mut v = (*filters).clone();
                                    v.push(Filter::BDRip);
                                    filter_to_req = v.clone();
                                    v
                                });
                            }

                            wasm_bindgen_futures::spawn_local(async move {
                                let get_torrent_full = get_torrent_full.clone();
                                let torrents = get_torrents(&*ttl_cheap_en, &*ttl_cheap_def, n as u16, &filter_to_req, *get_torrent_full).await;
                                log!(format!("{:#?}", &torrents));
                                let mut cur_t = (*torrent_data_state).clone();
                                cur_t[n-1] = Some(torrents);
                                torrent_data_state.set(cur_t);
                            });
                        })
                    };

                    let hevc_filter = {
                        let ttl_cheap_def = ttl_cheap_def.clone();
                        let ttl_cheap_en = ttl_cheap_en.clone();
                        let filters = filters.clone();
                        let torrent_data_state = torrent_data_state.clone();
                        let get_torrent_full = get_torrent_full.clone();
                        let theme = theme.clone();
                        Callback::from(move |_: MouseEvent| {
                            let ttl_cheap_def = ttl_cheap_def.clone();
                            let ttl_cheap_en = ttl_cheap_en.clone();
                            let torrent_data_state = torrent_data_state.clone();
                            let get_torrent_full = get_torrent_full.clone();
                            let theme = theme.clone();
                            let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("hevc-btn-{}", n)).unwrap();
                            if btn.get_attribute("class").unwrap().contains("active") {
                                btn.set_attribute("class", &format!("eps-filter hover-highlight {}", &theme));
                            } else {
                                btn.set_attribute("class", &format!("eps-filter-active hover-highlight {}", &theme));
                            }

                            {
                                let torrent_data_state = torrent_data_state.clone();
                                let mut v = (*torrent_data_state).clone();
                                v[n-1] = None;
                                torrent_data_state.set(v);
                            }

                            let filters = filters.clone();
                            log!("hevc filter clicked.");
                            let mut filter_to_req = vec![];

                            if (*filters).contains(&Filter::HEVC) {
                                filters.set({
                                    let v = (*filters).clone().into_iter().filter(|f| {
                                    *f != Filter::HEVC
                                }).collect::<Vec<Filter>>();
                                filter_to_req = v.clone();
                                v
                            });
                            } else {
                                filters.set({
                                    let mut v = (*filters).clone();
                                    v.push(Filter::HEVC);
                                    filter_to_req = v.clone();
                                    v
                                });
                            }

                            wasm_bindgen_futures::spawn_local(async move {
                                let get_torrent_full = get_torrent_full.clone();
                                let torrents = get_torrents(&*ttl_cheap_en, &*ttl_cheap_def, n as u16, &filter_to_req, *get_torrent_full).await;
                                log!(format!("{:#?}", &torrents));
                                let mut cur_t = (*torrent_data_state).clone();
                                cur_t[n-1] = Some(torrents);
                                torrent_data_state.set(cur_t);
                            });
                        })
                    };

                    let enhanced_result = {
                        let ttl_cheap_def = ttl_cheap_def.clone();
                        let ttl_cheap_en = ttl_cheap_en.clone();
                        let filters = filters.clone();
                        let torrent_data_state = torrent_data_state.clone();
                        let theme = theme.clone();
                        let get_torrent_full = get_torrent_full.clone();
                        Callback::from(move |_: MouseEvent| {
                            let ttl_cheap_def = ttl_cheap_def.clone();
                            let ttl_cheap_en = ttl_cheap_en.clone();
                            let torrent_data_state = torrent_data_state.clone();
                            let theme = theme.clone();
                            let get_torrent_full = get_torrent_full.clone();
                            let btn = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("enhanced-result-btn-{}", n)).unwrap();

                            if btn.get_attribute("class").unwrap().contains("active") {
                                btn.set_attribute("class", &format!("eps-filter hover-highlight {}", &theme));
                            } else {
                                btn.set_attribute("class", &format!("eps-filter-active hover-highlight {}", &theme));
                            }

                            {
                                let torrent_data_state = torrent_data_state.clone();
                                let mut v = (*torrent_data_state).clone();
                                v[n-1] = None;
                                torrent_data_state.set(v);
                            }

                            let filters = filters.clone();
                            log!("get_torrent_full enabled.");
                            let mut filter_to_req = vec![];

                            // if (*filters).contains(&Filter::HEVC) {
                            //     filters.set({
                            //         let v = (*filters).clone().into_iter().filter(|f| {
                            //         *f != Filter::HEVC
                            //     }).collect::<Vec<Filter>>();
                            //     filter_to_req = v.clone();
                            //     v
                            // });
                            // } else {
                            //     filters.set({
                            //         let mut v = (*filters).clone();
                            //         v.push(Filter::HEVC);
                            //         filter_to_req = v.clone();
                            //         v
                            //     });
                            // }
                            if (*get_torrent_full) == true {
                                get_torrent_full.set(false);
                                let filter_buttons = web_sys::window().unwrap().document().unwrap().get_elements_by_class_name("eps-filter-btn");
                                for i in (0..filter_buttons.length()) {
                                    let btn = filter_buttons.get_with_index(i).expect("Index out of bounds.").dyn_into::<HtmlButtonElement>().unwrap();
                                    // btn.set_disabled(true);
                                    let cur_class = btn.get_attribute("class")
                                        .unwrap()
                                        .split(' ')
                                        .chain(["cursor-disabled"])
                                        .filter_map(|s| {
                                            if s == "hover-highlight" {
                                                None
                                            } else if s == "eps-filter-active" {
                                                Some("eps-filter".to_string())
                                            } else {
                                                Some(s.to_string())
                                            }
                                        })
                                        .collect::<Vec<String>>();
                                    log!(format!("{:?}", &cur_class));
                                    btn.set_attribute("class", cur_class.join(" ").as_str());
                                    btn.set_attribute("title", "Enabled `Enhanced Result` to use the filters!");
                                    filters.set(vec![]);
                                }
                            } else {
                                alert("Turning on \"Enhanced Result\" will improve torrent search result by querying nyaa.si synonyms of the anime's title. This is especially effective for sequels and anime with long or multiple titles.\n\nNote: Torrent fetching will take more time. Be sure to visit nyaa.si directly if the results do not match your expectation.");
                                get_torrent_full.set(true);
                                let filter_buttons = web_sys::window().unwrap().document().unwrap().get_elements_by_class_name("eps-filter-btn");
                                for i in (0..filter_buttons.length()) {
                                    let btn = filter_buttons.get_with_index(i).expect("Index out of bounds.").dyn_into::<HtmlButtonElement>().unwrap();
                                    // btn.set_disabled(false);
                                    let cur_class = btn.get_attribute("class")
                                        .unwrap()
                                        .split(' ')
                                        .chain(["hover-highlight"])
                                        .filter_map(|s| {
                                            if s == "cursor-disabled" {
                                                None
                                            } else {
                                                Some(s.to_string())
                                            }
                                        })
                                        .collect::<Vec<String>>();
                                    btn.set_attribute("class", cur_class.join(" ").as_str());
                                    btn.remove_attribute("title");
                                }
                            }

                            wasm_bindgen_futures::spawn_local(async move {
                                let get_torrent_full = get_torrent_full.clone();
                                let torrents = get_torrents(&*ttl_cheap_en, &*ttl_cheap_def, n as u16, &filter_to_req, !(*get_torrent_full)).await;
                                log!(format!("{:#?}", &torrents));
                                let mut cur_t = (*torrent_data_state).clone();
                                cur_t[n-1] = Some(torrents);
                                torrent_data_state.set(cur_t);
                            });
                        })
                        
                    };

                    html!{
                        <dialog id={format!("eps-{}", n)} class={format!("eps-dwld {}", theme.clone())}>
                        <div class="dialog-close-button-container">
                            <h2 class="content-ttl">{"Torrents from Nyaa"}</h2>
                            <a class="a-btn dialog-close-button ripple" onclick={move |_| {
                                let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("eps-{}", n)).unwrap();
                                elem.set_attribute("class", &format!("eps-dwld {}", theme_cloned.clone())).unwrap();
                                let dialog = elem.dyn_into::<HtmlDialogElement>().unwrap();
                                dialog.close();
                                log!("closing dialog.");
                            }}><img class={format!("icon-{}", theme.clone())}src="./static/dialog_close.png" width="24px" height="24px"/>
                            </a>
                        </div>

                        <div class="eps-filter-wrapper">
                            <h3 class="eps-filter-title">{"Filters"}</h3>
                            <button id={format!("enhanced-result-btn-{}", n)} class={format!("eps-filter{} hover-highlight {}", {
                                if *get_torrent_full {
                                    "-active"
                                } else {
                                    ""
                                }
                            },theme.clone())} onclick={enhanced_result}>{"Enhanced Result"}
                            </button>
                            {
                                    FILTER_TYPES
                                        .iter()
                                        .zip(filter_callbacks.iter())
                                        .map(|(filter_type, callback)| {
                                            html!{
                                                <button id={format!("{}-btn-{}", filter_type.as_str(), n)} class={format!("eps-filter-btn eps-filter{} {} {}", {
                                                    if (*(filters.clone())).contains(&filter_type) {
                                                        "-active"
                                                    } else {
                                                        ""
                                                    }
                                                }, theme.clone(), {
                                                    if (*get_torrent_full) {
                                                        ""
                                                    } else {
                                                        "cursor-disabled"
                                                    }
                                                })} onclick={callback}>{filter_type.as_str()}
                                                </button>
                                            }
                                        })
                                        .collect::<Vec<Html>>()
                            }

                            // <button id={format!("bd-btn-{}", n)} class={format!("eps-filter{} hover-highlight {}", {
                            //     if (*(filters.clone())).contains(&Filter::BDRip) {
                            //         "-active"
                            //     } else {
                            //         ""
                            //     }
                            // }, theme.clone())} onclick={bdrip_filter}>{"BDRip"}</button>

                            // <button id={format!("hevc-btn-{}", n)} class={format!("eps-filter{} hover-highlight {}", {
                            //     if (*(filters.clone())).contains(&Filter::HEVC) {
                            //         "-active"
                            //     } else {
                            //         ""
                            //     }
                            // },theme.clone())} onclick={hevc_filter}>{"HEVC"}</button>
                        </div>

                        <div class="client-query">
                           <span>{"Custom Keyword"}</span><input type="text" id={format!("client-torrent-custom-query-{}", n)} class="client-torrent-custom-query" oninput={&update_client_query}/>
                        </div>

                        <section class="torrent-cards-wrapper">

                        {
                            if (n-1) < (*torrent_data_state).len() {
                                match (*torrent_data_state)[(n-1) as usize] {
                                    None => html!{<Loading/>},
                                    Some(ref v) => {
                                        if v.len() == 0 {
                                            html!{"No torrents are found."}
                                        } else {
                                            (v.clone()).into_iter().map(|torrent| {
                                                if (&torrent.title).to_lowercase().contains(&*client_query) {
                                                    html!{
                                                        <div class={format!("torrent-card obj-level-1 hover-highlight {}", theme.clone())}>
                                                            <article class="torrent-card-info cursor-arrow">
                                                                <section class="torrent-identity">
                                                                    <span class="torrent-title">
                                                                    <img class={format!("icon-{}", theme.clone())}src="./static/video.svg" width="14px" height="14px"/>
                                                                    <a class="u_onhover" href={torrent.link_view} target="_blank">{&(torrent.title)}</a></span>
                                                                </section>
                                                                <section class="torrent-links">
                                                                    <div class="torrent-download">
                                                                        <img class={format!("icon-{}", theme.clone())}src="./static/checkmark.svg" width="14px" height="14px"/>
                                                                    {&(torrent.download)}</div>
                                                                    <span class="torrent-size-mb">
                                                                    <img class={format!("icon-{}", theme.clone())}src="./static/diskette.svg" width="14px" height="14px"/>
                                                                    {&(torrent.size_mb)}</span>
                                                                </section>
                                                            </article>
                                                            <section class="torrent-download-buttons">
                                                                <div class="torrent-link-magnet"><a class="a-btn ripple" href={(torrent.link_magnet).clone()}>
                                                                    <img class={format!("icon-{}", theme.clone())}src="./static/torrent-magnet.svg" width="32px" height="32px"/>
                                                                </a></div>
                                                                <div class="torrent-link-torrent"><a class="a-btn ripple" href={(torrent.link_torrent).clone()}>
                                                                    <img class={format!("icon-{}", theme.clone())}src="./static/torrent-download.svg" width="32px" height="32px"/>
                                                                </a></div>
                                                            </section>
                                                        </div>
                                                    }
                                                } else {
                                                    html!{}
                                                }

                                            }).collect::<Html>()
                                        }
                                    }
                                }
                            } else {
                                html!{"Error retrieving episodes. Please refresh the page by pressing F5."}
                            }
    }
                        </section>

                        </dialog>
                    }
                }).collect::<Html>()}
                </div>


            </div>
        }
}
