use crate::prelude::*;

pub fn handle_theme_icon(cx: &AppContext) -> &'static str {
    if let Theme::Light = ((*cx).theme).clone() {
        "./static/moon.svg"
    } else if let Theme::Dark = ((*cx).theme).clone() {
        "./static/light.svg"
    } else {
        ""
    }
}

pub fn handle_title_language(cx: &AppContext, ao: &AnimeObj) -> String {
    match (*cx).get_langauge() {
        Language::EN => (*ao)
            .title_english
            .clone()
            .unwrap_or((*ao).titles[0].title.clone()),
        Language::JP => (*ao).title_japanese.clone().unwrap_or(
            (*ao)
                .title_english
                .clone()
                .unwrap_or((*ao).titles[0].title.clone()),
        ),
    }
}

pub fn handle_eps_language(cx: &AppContext, eo: &AnimeEpisode) -> String {
    match &(*cx).language {
        &Language::EN => (*eo).title.clone(),
        &Language::JP => (*eo)
            .title_japanese
            .as_ref()
            .unwrap_or(&(*eo).title)
            .to_string(),
    }
}
