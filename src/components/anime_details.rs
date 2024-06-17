use gloo::dialogs::alert;
use serde_json::json;
use wasm_bindgen::closure::Closure;
use web_sys::{js_sys::Date, Document, HtmlElement, Window};

use crate::prelude::*;

fn handle_year(year: Option<u32>) -> Html {
    match year {
        Some(n) => {
            html! {<div class="meta-header">{"Year"}<span class="meta-data">
                <span class="ad-year">{n}</span>
                </span></div>
            }
        }
        None => html! {},
    }
}

fn handle_rating_html(rating: Option<String>, scored_by: Option<u64>) -> Html {
    match rating {
        None => html! {},
        Some(r) => {
            if &r == "0" || &r == "0.0" {
                html! {}
            } else {
                html! {
                    <div class="meta-header">{"Rating"}<span class="meta-data"><span class="ad-rating">{r}<span class="fw-150">{crate::utils::data_handling::handle_rating(scored_by)}</span></span></span></div>
                }
            }
        }
    }
}

fn handle_studios(studios: Vec<MALObj>) -> Html {
    let n = studios.len();
    if n == 0 {
        return html! {};
    } else {
        html! {
            <div class="meta-header">{"Studios"}<span class="meta-data"><span class="ad-studios">{
                studios.into_iter().enumerate().map(|(m, studio)| {
                    if m+1 == n {
                        html! (<>{studio.name}</>)
                    } else {
                        html! {<>{studio.name}{ " | "}</>}
                    }
                }).collect::<Html>()
            }
            </span></span></div>
        }
    }
}

fn handle_synopsis(synopsis: Option<String>, theme: String) -> Html {
    match synopsis {
        None => html! {
        <div class="ad-synopsis">
        <div class="ad-section-header"><img id="synopsis-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/bookmark.svg"/><h2 id="ad-section-header" class="content-ttl">{"Synopsis"}</h2></div>
        {"Synopsis is unavailable."}
        </div>},
        Some(s) => html! {
            <div class="ad-synopsis">
            <div class="ad-section-header"><img id="synopsis-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/bookmark.svg"/><h2 id="ad-section-header" class="content-ttl">{"Synopsis"}</h2></div>
            <p>{s}</p>
            </div>
        },
    }
}

fn handle_anime_theme(themes: Vec<MALObj>) -> Html {
    let n = themes.len();
    let mut m = 1;
    if n == 0 {
        return html! {};
    } else {
        html! {
            <div class="meta-header">{"Theme"}<span class="meta-data"><span class="ad-themes">{
                themes.into_iter().map(|theme| {
                    if m == n {
                        html! (<>{theme.name}</>)
                    } else {
                        m += 1;
                        html! {<>{theme.name}{ ", "}</>}
                    }
                }).collect::<Html>()
            }
            </span></span></div>
        }
    }
}

pub fn handle_date(date: Option<String>) -> Html {
    match date {
        None => html! {">> handle_date: date is none."},
        Some(d) => {
            let date = chrono::DateTime::parse_from_str(&d, "%Y-%m-%dT%H:%M:%S%z");
            match date {
                Err(_) => html! {">> chrono: error parsing date."},
                Ok(dt) => {
                    let date_displayed = format!("{}", dt.format("%A, %e %B %Y"));
                    html! {<>
                        <div class="meta-header">{"Status "}<div class="meta-data">
                            {"Airing on "}<u>{&date_displayed}</u>
                        </div>
                        </div>
                    </>}
                }
            }
        }
    }
}

fn handle_eps(eps: usize) -> Html {
    match eps {
        0 => html! {},
        _ => {
            html! {
                <div class="meta-header">{"Episodes"}<span class="meta-data">{eps}</span></div>
            }
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub mal_id: u64,
}

#[function_component(Content)]
pub fn content(props: &Props) -> HtmlResult {
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let mal_id = props.mal_id;
    let nav = use_navigator().unwrap();

    use_effect_once(|| {
        apply_horizontal_scroll(&web_sys::window().expect("Window should be present."));
        || {}
    });

    let result = {
        use_future_with(mal_id, |_| async move {
            // log!("Requesting anime details...");
            let link_ao = format!("https://api.jikan.moe/v4/anime/{}/full", mal_id);
            let link_co = format!("https://api.jikan.moe/v4/anime/{}/characters", mal_id);
            let link_eps = format!("https://api.jikan.moe/v4/anime/{}/episodes", mal_id);
            let link_rec = format!("https://api.jikan.moe/v4/anime/{}/recommendations", mal_id);

            let ao_req = force_req::<AnimeObjFullAsQueryResult>(&link_ao);
            let char_req = force_req::<CharWrapper>(&link_co);
            let eps_req = force_req::<AnimeEpisodeWrapper>(&link_eps);
            let rec_req = force_req::<AnimeRecResult>(&link_rec);

            let (ao_result, char_result, eps_result, rec_result) =
                join!(ao_req, char_req, eps_req, rec_req);
            log!(format!("{:#?}", ao_result.as_ref().unwrap().data.aired));
            log!(format!("{:#?}", ao_result.as_ref().unwrap().data.status));
            let last_page = eps_result.as_ref().unwrap().pagination.last_visible_page;
            let eps_info = *ao_result
                .as_ref()
                .unwrap()
                .data
                .episodes
                .as_ref()
                .unwrap_or(&0);

            let mut total_episodes = 0;

            if eps_info == 0 {
                if last_page > 1 {
                    let link = format!("{}?page={}", &link_eps, last_page);
                    let n_last_page = force_req::<AnimeEpisodeWrapper>(&link)
                        .await
                        .unwrap()
                        .data
                        .len();
                    total_episodes = ((last_page - 1) * 100 + n_last_page as u32) as usize;
                } else {
                    total_episodes = eps_result.as_ref().unwrap().data.len();
                }
            } else {
                if let Some(t) = &ao_result.as_ref().unwrap().data.r#type.as_ref() {
                    if t.as_str() == "Movie" {
                        total_episodes = 0;
                    } else {
                        total_episodes = eps_info as usize;
                    }
                } else {
                    total_episodes = eps_info as usize;
                }
            }

            (
                ao_result,
                char_result,
                eps_result,
                rec_result,
                Ok::<usize, &'static str>(total_episodes),
            )
        })?
    };

    let html_result = match &(*result) {
        (Ok(ao), Ok(co), Ok(eo), Ok(ro), Ok(n)) => {
            log!(
                "Setting title to:",
                handle_title_language(&cx, &(ao.data.clone().into()))
            );
            web_sys::window().unwrap().document().unwrap().set_title(
                format!(
                    "{} | ANiNFO",
                    handle_title_language(&cx, &(ao.data.clone().into()))
                )
                .as_str(),
            );

            let add_to_favourite = {
                let ttl_en = ao.data.title_english.clone();
                let ttl_jp = ao.data.title_japanese.clone();
                let img_link = ao.data.images.webp.large_image_url.clone();
                let jwt = cx.jwt.clone();
                let uuid = cx.uuid.clone();
                let mal_id = props.mal_id.clone();
                let cx = cx.clone();
                Callback::from(move |_: MouseEvent| {
                    let ttl_en = ttl_en.clone();
                    let ttl_jp = ttl_jp.clone();
                    let img_link = img_link.clone();
                    let uuid = uuid.clone();
                    let mal_id = mal_id.clone();
                    let jwt = jwt.clone();
                    let cx = cx.clone();
                    wasm_bindgen_futures::spawn_local(
                        async move {
                            let result = reqwasm::http::Request::post(&format!("{}/api/v1/users/add_anime", BASE_URL))
                                .header("Authorization", &format!("Bearer {}", jwt.unwrap()))
                                .header("Content-Type", "application/json")
                                .body(serde_json::to_string(
                                    &UserAnimeSubmission {
                                        uuid: uuid.unwrap(),
                                        anime_id: mal_id as i32,
                                        anime_img: img_link.clone(),
                                        anime_ttl_en: ttl_en.clone().unwrap(),
                                        anime_ttl_jp: ttl_jp.clone(),
                                    }
                                ).unwrap()).send().await.unwrap();

                            if result.status() == 200 {
                                let mut v =  (*cx).fav_anime.clone().unwrap().into_iter().map(|ua_response| {
                                    ua_response.anime_id
                                }).collect::<Vec<i32>>();
                                v.push(mal_id as i32);
                                v.sort();

                                cx.dispatch(
                                    (*cx).update_fav_anime_into(Some({
                                        (*cx).fav_anime.clone().unwrap().into_iter().chain([UserAnimeResponse {
                                            anime_id: mal_id as i32,
                                            anime_img: img_link,
                                            anime_ttl_en: ttl_en.unwrap(),
                                            anime_ttl_jp: ttl_jp,
                                        }].into_iter()).collect::<Vec<UserAnimeResponse>>()
                                }))
                                    .update_fav_anime_id_into(Some(
                                        v
                                    ))
                                );
                                    alert("Successfully added this anime to Favourites!");
                                } else {
                                    alert("Failed adding this anime to Favourites. Please retry in a few moments.");
                                }
                        }
                    );
                })
            };

            let remove_from_favourite = {
                let jwt = cx.jwt.clone();
                let cx = cx.clone();
                let uuid = cx.uuid.clone();
                let mal_id = props.mal_id.clone();
                Callback::from(move |_: MouseEvent| {
                    let uuid = uuid.clone();
                    let mal_id = mal_id.clone();
                    let jwt = jwt.clone();
                    let cx = cx.clone();
                    wasm_bindgen_futures::spawn_local(
                        async move {
                            let result = reqwasm::http::Request::post(&format!("{}/api/v1/users/remove_anime/{}", BASE_URL, mal_id))
                                .header("Authorization", &format!("Bearer {}", jwt.unwrap()))
                                .header("Content-Type", "application/json")
                                .send().await.unwrap();
                            if result.status() == 200 {
                                let mut v =  (*cx).fav_anime.clone().unwrap().into_iter().filter_map(|ua_response| {
                                    if ua_response.anime_id == mal_id as i32 {
                                        None
                                    } else {
                                        Some(ua_response.anime_id)
                                    }
                                }).collect::<Vec<i32>>();
                                v.sort();
                                cx.dispatch(
                                    (*cx).update_fav_anime_into(Some((*cx).fav_anime.clone().unwrap().into_iter().filter(|ua_response| {
                                        if ua_response.anime_id == mal_id as i32 {
                                            false
                                        } else {
                                            true
                                        }
                                    }).collect::<Vec<UserAnimeResponse>>()))
                                    .update_fav_anime_id_into(Some(
                                        v
                                    ))
                                );
                                alert("Successfully removed this anime to Favourites!");
                            } else {
                                alert("Failed removing this anime to Favourites. Please retry in a few moments.");
                            }
                        }
                    );
                })
            };

            html! {
                <>
                    <div class={format!("anime-details-wrapper {}", theme)}>

                            // LARGE SCREEN
                        <div class={format!("anime-details-header {}", theme)}>
                            <h1 class="anime-details-title only-small-screen">{handle_title_language(&cx, &(ao.data.clone().into()))}</h1>
                            // <h1 class="anime-details-title only-large-screen">{handle_title_language(&cx, &(ao.data))}</h1>
                            <img loading="lazy" class="only-large-screen" id="anime-details-header-cover" src={extract_img_from_cx(&(ao.data.clone().into()))}/>
                            <div class="anime-details-header-info only-large-screen">
                                <h1 class="anime-details-title">{handle_title_language(&cx, &(ao.data.clone().into()))}</h1>
                                <article class="anime-meta">
                                {self::handle_year(ao.data.year.clone())}
                                {
                                    if let Some(AnimeStatus::NotYetAiring) = ao.data.status {
                                        self::handle_date(ao.data.aired.from.clone())
                                    } else {
                                        html!{}
                                    }
                                }
                                {self::handle_eps(*n)}
                                {self::handle_rating_html(ao.data.score.clone(), ao.data.scored_by.clone())}
                                {self::handle_studios(ao.data.studios.clone())}
                                {self::handle_anime_theme(ao.data.themes.clone())}
                                </article>
                                <article class={format!("genre-big {}", theme)}>
                                    {ao.data.genres.clone().into_iter().map(|genre_obj| {
                                        let nav = nav.clone();
                                        html!(<span class={format!("{} cursor-pointer u_onhover", theme)}><a onclick={
                                            let genre_name = genre_obj.name.clone();
                                            move |_| {
                                                nav.push(&Route::AnimeResultStd {
                                                    content_title: format!("Top {} Anime", genre_name),
                                                    page: 1,
                                                    url: StdResultType::Genre(genre_obj.mal_id)})
                                            }
                                        }>{genre_obj.name.clone()}</a></span>)
                                    }).collect::<Html>()}
                                </article>
                                <div class="ad-user-options">
                                {
                                    if let Some(_) = (*cx).fav_anime_id {
                                        if let Some(_) = binary_search(((*cx).fav_anime_id.as_ref().unwrap()), &(props.mal_id as i32)) {
                                            html!{
                                                <a class="a-btn ripple" onclick={remove_from_favourite}><img id="fav-icon" src="./static/heart_filled.svg" width="28px" height="28px" class="icon-dark"/></a>
                                            }
                                        } else {
                                            html!{
                                                <a class="a-btn ripple" onclick={add_to_favourite}><img id="fav-icon" src="./static/heart.svg" width="28px" height="28px" class="icon-dark"/></a>
                                            }
                                        }
                                    } else {
                                        html! {
                                            <a onclick={|_| {
                                                alert("You need to be logged in to add an anime to Favourites.");
                                            }}><img id="fav-icon" src="./static/heart.svg" width="28px" height="28px" class="icon-dark cursor-disabled"/></a>
                                        }
                                    }
                                }
                                </div>
                            </div>

                            // SMALL SCREEN
                            <div class="info-img-wrapper only-small-screen">
                                <img loading="lazy" id="anime-details-header-cover" src={extract_img_from_cx(&(ao.data.clone().into()))}/>
                                <div class="anime-details-header-info">
                                    <h1 class="anime-details-title">{handle_title_language(&cx, &(ao.data.clone().into()))}</h1>
                                    {self::handle_year(ao.data.year.clone())}
                                    {
                                        if let Some(AnimeStatus::NotYetAiring) = ao.data.status {
                                            self::handle_date(ao.data.aired.from.clone())
                                        } else {
                                            html!{}
                                        }
                                    }
                                    {self::handle_rating_html(ao.data.score.clone(), ao.data.scored_by.clone())}
                                    {self::handle_studios(ao.data.studios.clone())}
                                    {self::handle_anime_theme(ao.data.themes.clone())}
                                    <article class={format!("genre-big {}", theme)}>
                                        {ao.data.genres.clone().into_iter().map(|genre_obj| {
                                            html!(<span class={format!("{}", theme)}>{genre_obj.name.clone()}</span>)
                                        }).collect::<Html>()}
                                    </article>
                                </div>
                            </div>

                        </div>

                        {handle_synopsis(ao.data.synopsis.clone(), theme.clone())}

                        <div class={format!("streams-wrapper {}", theme)}>
                        <div class="ad-section-header">
                        <img loading="lazy" id="streams-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/stream_download.svg"/>
                        <h2 id="ad-section-header" class="content-ttl">{"Stream / Download"}</h2></div>
                            <div class="streams-btn-wrapper">
                            <a class={format!("stream-btn hover-highlight {}", theme)} target="_blank" rel="noopener noreferrer" href={format!("https://nyaa.si/?f=0&c=1_0&q={}&s=seeders&o=desc", ao.data.titles[0].clone().title)}>
                            <img loading="lazy" class={format!("icon-{}", theme)} src="./static/nyaa_light.png"/><p class="stream-caption"><span class="font-weight-150">{"Download from "}</span><b>{"nyaa.si"}</b></p></a>
                            <a class={format!("stream-btn hover-highlight {}", theme)} target="_blank" rel="noopener noreferrer" href={format!("https://kayoanime.com/?s={}", ao.data.titles[0].clone().title)}>
                            <img loading="lazy" class={format!("icon-{}", theme)} src="./static/kayoanime.png"/><p class="stream-caption"><span class="font-weight-150">{"Download from "}</span><b>{"Kayoanime"}</b></p></a>
                            <a class={format!("stream-btn hover-highlight {}", theme)} target="_blank" rel="noopener noreferrer" href={format!("https://aniwave.to/filter?keyword={}", ao.data.titles[0].clone().title)}>
                            <img loading="lazy" class={format!("icon-{}", theme)} src="./static/aw_light.png"/><p class="stream-caption"><span class="font-weight-150">{"Stream on"}</span><b>{" AniWave"}</b></p></a>
                            <a class={format!("stream-btn hover-highlight {}", theme)} target="_blank" rel="noopener noreferrer" href={format!("https://anix.to/filter?keyword={}", ao.data.titles[0].clone().title)}>
                            <img loading="lazy" class={format!("icon-{}", theme)} src="./static/anix.png"/><p class="stream-caption"><span class="font-weight-150">{"Stream on "}</span><b>{"Anix"}</b></p></a>
                            <a class={format!("stream-btn hover-highlight {}", theme)} target="_blank" rel="noopener noreferrer" href={format!("https://zorotv.com.in/?s={}", ao.data.titles[0].clone().title)}>
                            <img loading="lazy" class={format!("icon-{}", theme)} src="./static/zoro.png"/><p class="stream-caption"><span class="font-weight-150">{"Stream on "}</span><b>{"Zoro"}</b></p></a>
                            </div>
                        </div>

                        <div class="characters-wrapper" id="characters-wrapper">
                            <div class="ad-section-header">
                            <img id="characters-icon" loading="lazy" class={format!("ad-section-icon icon-{} hideable", theme.clone())} src="./static/character.png"/><h2 id="ad-section-header" class="content-ttl">{"Characters"}</h2></div>
                            {if (co.data).len() == 0 {
                                html!{<span class="no-result">{"Character data is unavailable."}</span>}
                            } else {
                                html!{
                                    <div class="char-card-styler">
                                    <div class="char-card-wrapper" id="char-card-wrapper">
                                    {into_char_cards(&(co.data))}
                                    </div>
                                    </div>
                                }
                            }}

                        </div>

                        <EpisodeCards
                            anime_ttl_def={ao.data.titles[0].title.clone()}
                            anime_ttl_en={ao.data.title_english.clone().unwrap_or(ao.data.titles[0].title.clone())}
                            eo={eo.data.clone()}
                            eps_total={*n as usize}
                            total_pages = {eo.pagination.last_visible_page as usize}
                            mal_id={ao.data.mal_id}
                        />

                        <section class="themesongs-wrapper">
                        <div class="ad-section-header">
                        <img loading="lazy" id="themesong-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/music.png"/><h2 id="ad-section-header" class="content-ttl">{"Theme Songs"}</h2></div>
                        {if ao.data.theme.openings.len() == 0 && ao.data.theme.endings.len() == 0 {
                            html!{<span class="no-result">{"Theme song data is unavailable."}</span>}
                        } else {
                            html!{
                                <>
                                <h3 class="content-subttl">{"Openings"}</h3>
                                <div class="themesongs-card-wrapper cursor-arrow">
                                {if ao.data.theme.openings.len() == 0 {
                                    html!{{"Anime opening data is unavailable."}}
                                } else {
                                    ao.data.theme.openings.iter().enumerate().map(|(i, s)| {
                                        if let Ok(r) = parse_themesong(s) {
                                            html!{<div class={format!("themesong-card obj-level-2 hover-highlight {}", theme.clone())}>
                                            <span class="themesong-header">
                                                <span class="themesong-no">{i+1}</span>
                                                <span class="themesong fw-300">
                                                    <a target="_blank" href={format!("https://www.youtube.com/results?search_query={} {}", &r.title, &r.artist)}>
                                                    <span class="themesong-title"><b>{&r.title}</b></span>{" - "}<span class="themesong-artist">{&r.artist}</span></a></span>
                                            </span>

                                            <div class="ts-dropdown-icon">
                                            <span class="ts-eps-range fw-300 hideable">
                                            { match r.eps.as_ref() {
                                                None => html!{},
                                                Some(s) => html!{s}
                                            }}
                                        </span>
                                            <a target="_blank" href={format!("https://www.youtube.com/results?search_query={} {}", &r.title, &r.artist)} class="a-btn ripple cursor-pointer">
                                                <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/youtube.svg"/>
                                            </a>
                                            <a target="_blank" href={format!("https://open.spotify.com/search/{} {}", &r.title, &r.artist)} class="a-btn ripple cursor-pointer">
                                                <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/spotify.svg"/>
                                            </a>
                                            </div>
                                            </div>}
                                        } else {
                                            html!{<div class={format!("themesong-card obj-level-2 hover-highlight {}", theme.clone())}>
                                            <span class="themesong fw-300">
                                                <a target="_blank" href={format!("https://www.youtube.com/results?search_query={}", s)}>
                                                {s}</a></span>
                                            <div class="ts-dropdown-icon">
                                            <a target="_blank" href={format!("https://www.youtube.com/results?search_query={}", s)} class="a-btn ripple cursor-pointer">
                                                <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/youtube.svg"/>
                                            </a>
                                            <a target="_blank" href={format!("https://open.spotify.com/search/{}", s)} class="a-btn ripple cursor-pointer">
                                            <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/spotify.svg"/>
                                            </a>
                                            </div>
                                            </div>}
                                        }
                                    }).collect::<Html>()
                                }}
                                </div>
                                <h3 class="content-subttl">{"Endings"}</h3>
                                <div class="themesongs-card-wrapper cursor-arrow">
                                {if ao.data.theme.endings.len() == 0 {
                                    html!{{"Anime ending data is unavailable."}}
                                } else {
                                    ao.data.theme.endings.iter().enumerate().map(|(i, s)| {
                                        if let Ok(r) = parse_themesong(s) {
                                            html!{<div class={format!("themesong-card obj-level-2 hover-highlight {}", theme.clone())}>
                                            <span class="themesong-header">
                                                <span class="themesong-no">{i+1}</span>
                                                <span class="themesong fw-300">
                                                    <a target="_blank" href={format!("https://www.youtube.com/results?search_query={} {}", &r.title, &r.artist)}>
                                                    <span class="themesong-title"><b>{&r.title}</b></span>{" - "}<span class="themesong-artist">{&r.artist}</span></a></span>
                                            </span>

                                            <div class="ts-dropdown-icon">
                                            <span class="ts-eps-range fw-300 hideable">
                                            { match &r.eps.as_ref() {
                                                &None => html!{},
                                                &Some(s) => html!{s}
                                            }}
                                            </span>
                                            <a target="_blank" href={format!("https://www.youtube.com/results?search_query={} {}", &r.title, &r.artist)} class="a-btn ripple cursor-pointer">
                                                <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/youtube.svg"/>
                                            </a>
                                            <a target="_blank" href={format!("https://open.spotify.com/search/{} {}", &r.title, &r.artist)} class="a-btn ripple cursor-pointer">
                                                <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/spotify.svg"/>
                                            </a>
                                            </div>
                                            </div>}
                                        } else {
                                            html!{<div class={format!("themesong-card obj-level-2 hover-highlight {}", theme.clone())}>
                                            <span class="themesong fw-300">
                                                <a target="_blank" href={format!("https://www.youtube.com/results?search_query={}", s)}>
                                                {s}</a></span>
                                            <div class="ts-dropdown-icon">
                                            <a target="_blank" href={format!("https://www.youtube.com/results?search_query={}", s)} class="a-btn ripple cursor-pointer">
                                                <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/youtube.svg"/>
                                            </a>
                                            <a target="_blank" href={format!("https://open.spotify.com/search/{}", s)} class="a-btn ripple cursor-pointer">
                                            <img loading="lazy" class={format!("eps-dropdown icon-{}", handle_theme(&cx))} src="./static/spotify.svg"/>
                                            </a>
                                            </div>
                                            </div>}
                                        }
                                    }).collect::<Html>()
                                }}
                                </div>
                                </>
                            }
                        }}

                    </section>

                        <div class="recommendation-wrapper" id="recommendation-wrapper">
                        <div class="ad-section-header">
                        <img loading="lazy" id="recommendation-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/download.png"/><h2 id="ad-section-header" class="content-ttl">{"Similar Anime"}</h2></div>
                        {into_anime_cards_minimal(&(ro.data))}
                        </div>

                    <CommentSection anime_id={mal_id as i32}/>

                    </div>
                </>
            }
        }
        _ => html! {"Uncatched exception."},
    };

    Ok(html!({ html_result }))
}

fn apply_horizontal_scroll(win: &Window) {
    let doc = win.document().unwrap();
    let mut last_call_rec = Date::now();
    let last_call_char =  Date::now();

    let rec_c = Closure::new(
        {
            let win = win.clone();
            Box::new(
                move |ev: Event| {
                    ev.prevent_default();
                    let doc = win.document().unwrap();
                    if Date::now() > last_call_rec + 0f64 {
                        last_call_rec = Date::now();
                        let scroll = ev.dyn_into::<WheelEvent>().expect("Valid cast.");
                        let rec_wrapper = doc.get_element_by_id("ani-rec-card-wrapper");
                        if let Some(e) = rec_wrapper {
                            log!("scrolling!");
                            e.scroll_by_with_x_and_y(scroll.delta_y()*3.5f64+scroll.delta_x()*3.5f64, 0f64);
                        } else {
                            return ()
                        }
                    } else {
                        let scroll = ev.dyn_into::<WheelEvent>().expect("Valid cast.");
                        let rec_wrapper = doc.get_element_by_id("ani-rec-card-wrapper");
                        if let Some(e) = rec_wrapper {
                            log!("scrolling!");
                            e.scroll_by_with_x_and_y(scroll.delta_y()*3.5f64+scroll.delta_x()*3.5f64, 0f64);
                        } else {
                            return ()
                        }
                        log!("throttled!");
                    }
                }
            ) as Box<dyn FnMut(_)>
        }

    );

    let char_c = Closure::new(
        {
            let doc = doc.clone();
            Box::new(
                move |ev: Event| {
                    ev.prevent_default();
                    let scroll = ev.dyn_into::<WheelEvent>().expect("Valid cast.");
                    let char_wrapper = doc.get_element_by_id("char-card-wrapper");
                    if let Some(e) = char_wrapper {
                        e.scroll_by_with_x_and_y(scroll.delta_y()*3.5f64+scroll.delta_x()*4f64, 0f64);
                        log!("scrolling!");
                    } else {
                        return ()
                    }
                }
            ) as Box<dyn FnMut(_)>
        }

    );

    gloo::timers::callback::Timeout::new(200, move || {
        let wrapper_1 = doc.get_element_by_id("ani-rec-card-wrapper");
        if let Some(e) = wrapper_1 {
            e.dyn_into::<HtmlElement>().expect("Valid cast.").set_onwheel(Some(rec_c.as_ref().unchecked_ref()));
        }

        let wrapper_2 = doc.get_element_by_id("char-card-wrapper");
        if let Some(e) = wrapper_2 {
            e.dyn_into::<HtmlElement>().expect("Valid cast.").set_onwheel(Some(char_c.as_ref().unchecked_ref()));
        }

        char_c.forget();
        rec_c.forget();
    }).forget();
}

#[function_component(AnimeDetails)]
pub fn anime_details(props: &Props) -> Html {
    
    html! {
        <Suspense fallback={html!(<Loading/>)}>
            <Content mal_id={props.mal_id}/>
        </Suspense>
    }
}
