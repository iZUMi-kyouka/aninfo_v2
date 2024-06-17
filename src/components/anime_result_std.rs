use crate::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct StdResultProp {
    pub page: u32,
    pub content_title: String,
    pub url: StdResultType,
}

#[function_component(Content)]
fn content(props: &StdResultProp) -> HtmlResult {
    let cx = use_context::<AppContext>().unwrap();
    let nav = use_navigator().unwrap();
    let props = (*props).clone();
    let time = use_state(|| "".to_string());

    let result: UseFutureHandle<Result<QueryResult, gloo::net::Error>> = {
        let cx = cx.clone();
        let props = props.clone();

        use_future_with(
            (
                (*cx).nsfw,
                (*cx).cur_page,
                (*cx).language,
                props.url.clone(),
            ),
            |_| async move {
                let link = format!(
                    "{}&page={}&sfw={}",
                    props.url.link(),
                    props.page,
                    !(*cx).nsfw
                );
                force_req::<QueryResult>(&link).await
            },
        )?
    };

    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let cur_time = Utc::now();
            let year = format!("{}", cur_time.format("%Y"));
            let season = season_from_month(&format!("{}", cur_time.format("%m")));
            time.set(format!("{} {}", season, year));
        })
    }

    let html_result = match (*result).as_ref() {
        Ok(v) => {
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .set_title(format!("No Result | ANiNFO").as_str());
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
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .set_title(format!("{} | ANiNFO", &(props.content_title)).as_str());
                html! {
                    <div class="search-result-wrapper" onload={|_| {log!("result-wrapper loaded.")}}>
                        <div class="content-header">
                            <span><h2 class="content-ttl vc-text">{&(props.content_title)}</h2></span>
                        </div>
                        <PageButtonsStd pagination={(*v).pagination.clone()} url={props.url.clone()} content_title={props.content_title.clone()} page={props.page}/>
                        <div class="cards-wrapper">
                            {into_anime_cards(&(v.data))}
                        </div>
                        <PageButtonsStd pagination={(*v).pagination.clone()} url={props.url} content_title={props.content_title.clone()} page={props.page}/>
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

#[function_component(AnimeResultStd)]
pub fn search_result(props: &StdResultProp) -> Html {
    html! {
        <Suspense fallback={html!(<Loading/>)}>
            <Content page={props.page} content_title={props.content_title.clone()} url={props.url.clone()}/>
        </Suspense>
    }
}
