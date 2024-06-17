use crate::prelude::*;

#[function_component]
fn TopAnime() -> HtmlResult {
    let result: UseFutureHandle<(
        Result<QueryResult, gloo::net::Error>,
        Result<QueryResult, gloo::net::Error>,
    )> = use_future(|| async {
        let seasonal = fetch_data_into::<QueryResult>("https://api.jikan.moe/v4/seasons/now").await;
        let top = fetch_data_into::<QueryResult>("https://api.jikan.moe/v4/top/anime").await;
        (seasonal, top)
    })?;

    let html_result = match (&(*result)).clone() {
        (Ok(seasonal), Ok(top)) => {
            let seasonal = seasonal.clone().data;
            let top = top.clone().data;
            html! {
            <>
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
            </>
            }
        }
        _ => {
            html! {"H"}
        }
    };

    Ok(html!({ html_result }))
}

#[function_component(BackupComponent)]
pub fn backup_component() -> Html {
    let fallback = html! {<Loading/>};

    html! {
        <>
            <Suspense {fallback}>
                <TopAnime/>
            </Suspense>
        </>
    }
}
