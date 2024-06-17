use std::error::Error;

use gloo::dialogs::alert;
use web_sys::{js_sys::Math::random, HtmlDialogElement, HtmlInputElement};

use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CommentProps {
    pub anime_id: i32,
}

#[function_component]
fn Content(props: &CommentProps) -> HtmlResult {
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let anime_id = props.anime_id;
    let nav = use_navigator().unwrap();
    let comment_status = use_state(|| random());

    let result = use_future_with(anime_id, |_| async move {
        let url = format!("{}/api/v1/anime/{}/comment", BASE_URL, anime_id);
        let comments = force_req::<Vec<AnimeCommentGet>>(&url).await;

        comments
    })?;

    let comment_field = use_state(|| "".to_string());

    let force_rerender = use_force_update();
    let onsubmit_comment = use_state(move || 
        Callback::from(move |_: MouseEvent| {
            log!("Rerendering...");
            force_rerender.force_update();
    }));

    let submit_post = {
        let comment_status = comment_status.clone();
        let comment_field = comment_field.clone();
        let cx = cx.clone();
        let theme = theme.clone();
        let nav = nav.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let cx = cx.clone();
            let theme = theme.clone();
            let nav = nav.clone();
            let comment_field = comment_field.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let theme = theme.clone();
                let auth = format!("Bearer {}", cx.jwt.as_ref().unwrap());
                let url = format!("{}/api/v1/anime/{}/comment", BASE_URL, anime_id);
                let comment = AnimeCommentPost {
                    comment: (*comment_field).clone(),
                };

                let result = reqwasm::http::Request::post(&url)
                    .header("content-type", "application/json")
                    .header("authorization", &auth)
                    .body(serde_json::to_string(&comment).unwrap())
                    .send()
                    .await;
                
                match result {
                    Ok(r) => {
                        if r.status() == 201 || r.status() == 200 {
                            alert("Your comment has been posted successfully.");
                        } else {
                            alert("Failed to post the commen    t. Please try to post your comment agian and relogin if the issue persists.");
                        }
                    },
                    Err(_) => {
                        alert("Failed to post the comment. Plese try to post your comment again.")
                    }
                }

                let elem = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&format!("add-comment-popup"))
                    .unwrap();
                elem.set_attribute("class", &format!("add-comment {}", &theme))
                    .unwrap();
                let dialog = elem.dyn_into::<HtmlDialogElement>().unwrap();
                dialog.close();
                nav.back();
                nav.forward();
            });
            comment_status.set(random());
        })
    };

    let comment_oninput = {
        let comment_field = comment_field.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target_unchecked_into::<HtmlInputElement>().value();
            comment_field.set(val);
        })
    };

    let theme_cloned = theme.clone();
    let html_result = match &(*result) {
        Err(_) => html!({ "Uncatched error." }),
        Ok(comments) => {
            html! {
                    <>
                    <dialog id="add-comment-popup" class={format!("add-comment {}", theme.clone())}>
                        <div class="dialog-close-button-container">
                            <h2 class="content-ttl">{"Add a Comment"}</h2>
                            <a class="a-btn dialog-close-button ripple" onclick={move |_| {
                                let elem = web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("add-comment-popup")).unwrap();
                                elem.set_attribute("class", &format!("add-comment {}", theme_cloned.clone())).unwrap();
                                let dialog = elem.dyn_into::<HtmlDialogElement>().unwrap();
                                dialog.close();
                                log!("closing dialog.");
                            }}><img class={format!("icon-{}", theme.clone())}src="./static/dialog_close.png" width="24px" height="24px"/>
                            </a>
                        </div>

                        <form id="post-comment" class="comment-form" onsubmit={submit_post}>
                            <textarea class={format!("{} comment-form-textarea", &theme)} row={"100%"} oninput={comment_oninput}/>
                            <input id="post-button" type="submit" class="hidden"/>
                        </form>

                        <div class={format!("full-width-btn cursor-pointer hover-highlight obj-level-1 {}", &theme)}>
                            <label for ="post-button">
                            <a class="a-btn" type="submit" form="post-comment" onclick={&*onsubmit_comment}>{"Post"}</a>
                            </label>
                        </div>
                    </dialog>
                    {
                        if comments.len() == 0 {
                            html! {
                                <span class="no-result">{"There are no comments for this anime yet."}</span>
                            }
                        } else {
                            html!{
                                <div class="comments-wrapper">
                                // <p style="display: hidden;">{*comment_status}</p>
                                {
                                    comments.into_iter().map(|c| {
                                        html!{  <>
                                                // <p>{*comment_status.clone()}</p>
                                                <div class={format!("comment-card obj-level-1 {}", &theme)}>
                                                    <span class="username">{&c.username}</span>
                                                    <span class="date">{&c.date}</span>
                                                    <span class="comment">{&c.comment}</span>
                                                </div>
                                                </>
                                        }
                                    }).collect::<Html>()
                                }
                                </div>
                            }

                        }
                    }
            </>
            }
        }
    };

    Ok(html! 
        {
            <>
            {html_result}
            </>
        })
}

#[function_component]
pub fn CommentSection(props: &CommentProps) -> Html {
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    let anime_id = props.anime_id;
    let nav = use_navigator().unwrap();

    let trigger = use_force_update();
    let onrefresh = use_state(move || Callback::from(move |_:MouseEvent| trigger.force_update()));

    let add_comment = {
        let cx = cx.clone();
        let theme = theme.clone();
        if let None = cx.jwt.as_ref() {
            Callback::from(|_: MouseEvent| {
                alert("You need to be logged in to post a comment. \
                \nOpen the left navigation bar (by clicking on the three stripes icon) to log in or create an account.");
            })
        } else {
            Callback::from(move |_: MouseEvent| {
                log!("clicked.");
                let elem = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("add-comment-popup")
                    .unwrap();
                let cur_class = elem.get_attribute("class").unwrap();
                elem.set_attribute("class", &format!("add-comment-active {}", &theme))
                    .unwrap();
                let elem = elem.dyn_into::<HtmlDialogElement>().unwrap();
                elem.show();
            })
        }
    };

    html! {
        <section class="ad-comment-wrapper">
            <div class="ad-section-header">
                <div class="ad-section-header-left">
                    <img loading="lazy" id="comment-icon"  class={format!("ad-section-icon hideable icon-{}", theme.clone())} src="./static/comment.svg"/>
                    <h2 id="ad-section-header" class="content-ttl">{"Comments"}</h2>
                </div>
                <div class="ad-section-header-right">
                    <a class="a-btn ripple" onclick={add_comment}>
                        <img id="plus-icon" src="./static/plus.svg" height="36px" class={format!("icon-{}", &theme)}/>
                    </a>
                </div>
            </div>
            <Suspense fallback={html!{<Loading/>}}>
                <Content anime_id={props.anime_id}/>
            </Suspense>
        </section>
    }
}
