use crate::prelude::*;

#[function_component(Content)]
fn content() -> HtmlResult {
    let cx = use_context::<AppContext>().unwrap();

    let result: UseFutureHandle<(
        Result<QueryResult, gloo::net::Error>,
        Result<QueryResult, gloo::net::Error>,
    )> = use_future_with((*cx).clone(), |_| async move {
        if let Some(t) = (*cx).cache.home_page_result.clone() {
            (Ok(t.0), Ok(t.1))
        } else {
            let seasonal = force_req::<QueryResult>("https://api.jikan.moe/v4/seasons/now").await;
            let top = force_req::<QueryResult>("https://api.jikan.moe/v4/top/anime").await;
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

    let html_result = match (&(*result)) {
        (Ok(seasonal), Ok(top)) => {
            let seasonal = seasonal.clone().data;
            let top = top.clone().data;
            html! {
            <>
            <div class="home-wrapper">
            <div class="content-header">
            <span><h2 class="content-ttl vc-text">{"Winter 2024's selection"}</h2></span>
            </div>
            <div class="cards-wrapper b-y">
            {
                seasonal.into_iter().map(|anime_obj| {
                    html! (<AnimeCard anime_obj={anime_obj.clone()} />)
                }).collect::<Html>()
            }
            </div>

            <div class="content-header">
            <span><h2 class="content-ttl vc-text">{"Anime of All Time"}</h2></span>
            </div>
            <div class="cards-wrapper b-y">
            {
                top.into_iter().map(|anime_obj| {
                    html! (<AnimeCard anime_obj={anime_obj.clone()} />)
                }).collect::<Html>()
            }
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

#[function_component(TestComponent)]
pub fn test_component() -> Html {
    let fallback = html! {<Loading/>};

    html! {
        <>
            <Suspense {fallback}>
                <Content/>
            </Suspense>
        </>
    }
}
