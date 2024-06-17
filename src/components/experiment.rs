use crate::prelude::*;
use std::time::Duration;
use tokio::join;

const URL: &str = "https://en.wikipedia.org/w/api.php?\
                   action=query&origin=*&format=json&generator=search&\
                   gsrnamespace=0&gsrlimit=5&gsrsearch='New_England_Patriots'";

#[function_component]
fn WikipediaSearch() -> HtmlResult {
    let cx = use_context::<AppContext>().unwrap();

    let cx_c = cx.clone();
    let res: UseFutureHandle<Result<QueryResult, gloo::net::Error>> = use_future(|| async move {
        let ctx_changed = (*cx_c).has_changed();
        let new_ctx = ctx_changed.1.clone();
        let r1 = force_req::<QueryResult>("https://api.jikan.moe/v4/top/anime").await;

        r1
    })?;
    let result_html = match *res {
        Ok(ref res) => html! { {into_anime_cards(res.clone().data.as_ref())} },
        Err(ref failure) => failure.to_string().into(),
    };

    use_effect(|| {
        let elem = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("exptl")
            .unwrap();
        elem.set_attribute("class", "exptl-used-active");

        || {}
    });

    Ok(html! {
        <div class="exptl">
            {"Search Result"}
            {result_html}
        </div>
    })
}

#[function_component(Experiment)]
pub fn experiment() -> Html {
    html! {
    <div id="exptl" class="exptl-used">
    <Suspense fallback={html!(<Loading/>)}>
        <WikipediaSearch/>
    </Suspense>
    </div>
    }
}
