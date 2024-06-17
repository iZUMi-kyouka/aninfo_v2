use crate::prelude::*;

pub fn handle_rating(scored_by: Option<u64>) -> String {
    match scored_by {
        None => "".to_string(),
        Some(mut n) => {
            if n >= 1000000 {
                n /= 1000000;
                format!(" ({}m)", n)
            } else if n >= 1000 {
                n /= 1000;
                format!(" ({}k)", n)
            } else {
                format!(" ({})", n)
            }
        }
    }
}

pub fn handle_long_title(s: &str, cx: &AppContext) -> Html {
    match (*cx).language {
        Language::EN => {
            if s.len() >= 65 {
                return html!(<span class="condensed-more">{s}</span>);
            } else if s.len() >= 50 {
                return html!(<span class="condensed">{s}</span>);
            } else {
                return html!({ s });
            }
        }
        Language::JP => {
            if s.len() >= 40 {
                return html!(<span class="condensed-more">{s}</span>);
            } else if s.len() >= 30 {
                return html!(<span class="condensed">{s}</span>);
            } else {
                return html!({ s });
            }
        }
    }
}

pub fn handle_long_title_compact(s: &str, cx: &AppContext) -> Html {
    match (*cx).language {
        Language::EN => {
            if s.len() >= 50 {
                return html!(<span class="condensed-more">{s}</span>);
            } else if s.len() >= 35 {
                return html!(<span class="condensed">{s}</span>);
            } else {
                return html!({ s });
            }
        }
        Language::JP => {
            if s.len() >= 30 {
                return html!(<span class="condensed-more">{s}</span>);
            } else if s.len() >= 20 {
                return html!(<span class="condensed">{s}</span>);
            } else {
                return html!({ s });
            }
        }
    }
}

pub fn handle_long_name(s: &str) -> Html {
    let chars = s.chars().collect::<Vec<char>>();
    if chars.len() > 24 {
        html!(<span class="condensed-more">{format!("{}", &chars.iter().collect::<String>())}</span>)
    } else if chars.len() > 18 {
        html!(<span class="condensed">{format!("{}", &chars.iter().collect::<String>())}</span>)
    } else {
        html! {{s}}
    }
}

pub fn handle_theme(app_ctx: &AppContext) -> String {
    if let Theme::Light = (*app_ctx).theme {
        "light".to_string()
    } else if let Theme::Dark = (*app_ctx).theme {
        "dark".to_string()
    } else {
        "".to_owned()
    }
}

pub fn handle_nsfw(app_ctx: &AppContext) -> &'static str {
    if app_ctx.nsfw {
        "&sfw=false"
    } else {
        "&sfw=true"
    }
}

pub fn extract_title_from_cx(ao: &AnimeObj, cx: &AppContext) -> String {
    let pref_lang = &(cx.language);
    match pref_lang {
        &Language::EN => {
            if let Some(t) = &((*ao).title_english) {
                return t.to_string();
            } else {
                return ((*ao).titles[0].title).to_string();
            }
        }
        &Language::JP => {
            if let Some(t) = &((*ao).title_english) {
                return t.to_string();
            } else {
                return ((*ao).titles[0].title).to_string();
            }
        }
    }
}

pub fn season_from_month(m: &str) -> &'static str {
    match m {
        "01" | "02" | "03" => "Winter",
        "04" | "05" | "06" => "Spring",
        "07" | "08" | "09" => "Summer",
        "10" | "11" | "12" => "Fall",
        _ => "Invalid Season",
    }
}

pub fn extract_img_from_cx(ao: &AnimeObj) -> String {
    ao.images
        .webp
        .image_url
        .clone()
        .unwrap_or(ao.images.jpg.image_url.clone().unwrap())
}
