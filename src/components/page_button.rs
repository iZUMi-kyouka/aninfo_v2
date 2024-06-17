use crate::prelude::*;
use web_sys::{DomRectReadOnly, HtmlCollection};

#[derive(Properties, PartialEq, Clone)]
pub struct PageButtonsStdProp {
    pub page: u32,
    pub content_title: String,
    pub url: StdResultType,
    pub pagination: PaginationObj,
}

#[function_component]
pub fn PageButtonsStd(props: &PageButtonsStdProp) -> Html {
    {
        use_effect(|| {
            // log!("page_btn_hook fired.");
            wasm_bindgen_futures::spawn_local(async {
                while let None = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("page-btn-selected")
                {
                    // log!("while loop.");
                    gloo::timers::future::TimeoutFuture::new(50).await;
                }
                let selected_btn = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("page-btn-selected")
                    .unwrap();
                let dom_rect = selected_btn
                    .get_bounding_client_rect()
                    .dyn_into::<DomRectReadOnly>()
                    .unwrap();
                let left = dom_rect.left();
                let vw = web_sys::window()
                    .unwrap()
                    .inner_width()
                    .unwrap()
                    .as_f64()
                    .unwrap();
                log!(format!("left: {}", left));
                log!(format!("vw: {}", vw));

                while web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_class_name("page-btn-wrapper")
                    .length()
                    == 0
                {
                    gloo::timers::future::TimeoutFuture::new(50).await;
                }

                if vw <= 1000f64 {
                    if left > vw {
                        let btn_wrappers: HtmlCollection = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_elements_by_class_name("page-btn-wrapper");
                        (0..btn_wrappers.length()).into_iter().for_each(|n| {
                            let wrapper = btn_wrappers.get_with_index(n).unwrap();
                            wrapper.scroll_by_with_x_and_y(left - (vw / 2f64) + 20f64, 0f64);
                        })
                    }
                } else {
                    if left > (vw / 2f64) + 400f64 as f64 {
                        let btn_wrappers: HtmlCollection = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_elements_by_class_name("page-btn-wrapper");
                        (0..btn_wrappers.length()).into_iter().for_each(|n| {
                            let wrapper = btn_wrappers.get_with_index(n).unwrap();
                            wrapper.scroll_by_with_x_and_y(left - (vw / 2f64) + 20f64, 0f64);
                        })
                    }
                }
            });
        });
    }

    let nav = use_navigator().unwrap();
    let cx = use_context::<AppContext>().unwrap();
    html! {
        <div class="page-btn-wrapper">
        {into_page_btns_std(cx.clone(), props.pagination.clone(), nav.clone(), props.url.clone(), props.page, &(props.content_title))}
        </div>
    }
}
