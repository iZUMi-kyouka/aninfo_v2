use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomeNew,
    #[at("/about")]
    About,
    #[at("/privacy_policy")]
    PrivacyPolicy,
    #[at("/search/:q/:page")]
    SearchResult { q: String, page: u32 },
    #[at("/search/:q")]
    SearchResultNoPage { q: String },
    #[at("/anime/:mal_id")]
    AnimeDetails { mal_id: u64 },
    #[at("/exp")]
    Experiment,
    #[at("/backup")]
    Backup,
    #[at("/debug")]
    Debug,
    #[at("/error/:app_err")]
    ErrorPage { app_err: AppErr },
    #[at("/loading")]
    Loading,
    #[at("/home")]
    Home,
    #[at("/anime/:content_title/:page/:url")]
    AnimeResultStd {
        content_title: String,
        page: u32,
        url: StdResultType,
    },
    #[at("/anime/explore")]
    ExploreAnime,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<TestComponent/>},
        Route::About => html! {<About/>},
        Route::PrivacyPolicy => html! {<PrivacyPolicy/>},
        Route::SearchResult { q, page } => html! {<SearchResult {q} {page}/>},
        Route::SearchResultNoPage { q } => html! {<SearchResult {q} page=1/>},
        Route::AnimeDetails { mal_id } => html! {<AnimeDetails {mal_id}/>},
        Route::Experiment => html! {<Experiment/>},
        Route::Backup => html! {<BackupComponent/>},
        Route::Debug => html! {<ContextDebugger/>},
        Route::ErrorPage { app_err } => html! {<ErrorPage {app_err}/>},
        Route::Loading => html! {<Loading/>},
        Route::HomeNew => html! {<Home/>},
        Route::AnimeResultStd {
            content_title,
            page,
            url,
        } => html! {<AnimeResultStd {url} {content_title} {page}/>},
        Route::NotFound => html! {<ErrorPage app_err={AppErr::not_found()}/>},
        Route::ExploreAnime => html! {<ExploreAnime/>},
    }
}
