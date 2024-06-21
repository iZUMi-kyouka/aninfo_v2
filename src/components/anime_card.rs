use crate::prelude::*;
use web_sys::js_sys;

#[derive(Properties, PartialEq, Clone)]
pub struct AnimeCardProp {
    pub anime_obj: AnimeObj,
}

#[function_component(AnimeCard)]
pub fn anime_card(props: &AnimeCardProp) -> Html {
    let ao = props.anime_obj.clone();
    let nav = use_navigator().unwrap();
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let mal_id = ao.mal_id;


    let go_to_detail = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::AnimeDetails { mal_id });
            web_sys::window()
                .expect("Missing window")
                .scroll_to_with_x_and_y(0f64, 0f64);
        })
    };

    let window = use_window_size();


    html! {
        <div class={format!("card hover-highlight b-r cursor-pointer {}", theme)} onclick={go_to_detail.clone()}>
            <img class="anime-cover cursor-pointer" id={format!("{}", &(ao.titles[0].title))} onclick={go_to_detail.clone()} width="inherit" loading="lazy" src={ao.images.clone().webp.image_url.unwrap_or(ao.images.clone().jpg.image_url.unwrap())}/>
            <article class="anime-info">
                <h4 class="title">
                    <a class="cursor-pointer" onclick={go_to_detail}>{
                        if window.0 <= 750f64 {
                            handle_long_title_compact(&handle_title_language(&cx, &ao), &cx)
                        } else {
                            handle_long_title(&handle_title_language(&cx, &ao), &cx)
                        }
                    }</a>
                </h4>
                {handle_year(&cx, ao.year.clone())}
                {if let None = ao.score.as_ref() {
                    html!{}
                } else if let "0" = ao.score.as_ref().unwrap().as_str() {
                    html!{}
                } else {
                    html!{<p class="rating"><img loading="lazy" class={format!("icon-{}", theme)} id="rating" src="./static/rating.svg"/>{ao.score.as_ref()}{handle_rating(ao.scored_by.clone())}</p>}
                }}

                // <p class="airing-status"><img id="airing-status" src="./static/tv.svg"/>{ao.status.as_ref().unwrap_or(&"Status Unavailable".to_string())}</p>
                {handle_studios(&cx, ao.studios.clone(), nav.clone())}
                <article class="genre">
                    // <span><p>Action</p></span>
                    // <span><p>Adventure</p></span>
                    // <span><p>Comedy</p></span>
                    // <span><p>Fantasy</p></span>
                    // <span><p>Psychological</p></span>
                    // <span><p>Thriller</p></span>
                    {ao.genres.into_iter().map(|genre_obj| {
                        let nav = nav.clone();
                        html!(<span class={format!("{} cursor-pointer hover-bold", theme)}><a onclick={
                            let genre_name = genre_obj.name.clone();
                            move |e: MouseEvent| {
                                e.stop_propagation();
                                nav.push(&Route::AnimeResultStd {
                                    content_title: format!("Top {} Anime", genre_name),
                                    page: 1,
                                    url: StdResultType::Genre(genre_obj.mal_id)})
                            }
                        }>{genre_obj.name.clone()}</a></span>)
                    }).collect::<Html>()}
                </article>
            </article>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct AnimeCardMinimalProp {
    pub ao: AnimeRecObj,
}

#[function_component(AnimeCardMinimal)]
pub fn anime_card(props: &AnimeCardMinimalProp) -> Html {
    let nav = use_navigator().unwrap();
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let mal_id = props.ao.mal_id;

    let go_to_detail = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::AnimeDetails {
                mal_id: mal_id as u64,
            });
            web_sys::window()
                .expect("Missing window")
                .scroll_to_with_x_and_y(0f64, 0f64);
        })
    };

    html! {
        <div class={format!("card-min hover-highlight b-r {}", theme)}>
            <img class="anime-cover-min cursor-pointer" id={format!("{}", &(props.ao.title))} onclick={go_to_detail.clone()} width="inherit" loading="lazy" src={props.ao.images.clone().webp.image_url.unwrap_or(props.ao.images.clone().jpg.image_url.unwrap())}/>
            <article class="anime-info-min">
                <h4 class="title-min"><a class="cursor-pointer" onclick={go_to_detail}>{handle_long_title_compact(props.ao.title.clone().as_ref(), &cx)}</a></h4>
                // <p class="airing-status"><img id="airing-status" src="./static/tv.svg"/>{ao.status.as_ref().unwrap_or(&"Status Unavailable".to_string())}</p>
            </article>
        </div>
    }
}
