use std::{fmt::Display, str::FromStr};

use crate::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum AppErr {
    Js(ErrMsg),
    Gloo(ErrMsg),
    Serde(ErrMsg),
    NotFound(ErrMsg),
}

impl FromStr for AppErr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..2] {
            "js" => {
                if s.len() == 2 {
                    return Ok(AppErr::js_default());
                } else {
                    return Ok(AppErr::Js(ErrMsg(s[2..].to_string())));
                }
            }
            "gl" => {
                if s.len() == 2 {
                    return Ok(AppErr::gloo_default());
                } else {
                    return Ok(AppErr::Gloo(ErrMsg(s[2..].to_string())));
                }
            }
            "sd" => {
                if s.len() == 2 {
                    return Ok(AppErr::js_default());
                } else {
                    return Ok(AppErr::Serde(ErrMsg(s[2..].to_string())));
                }
            }
            "nf" => {
                return Ok(AppErr::NotFound(ErrMsg(
                    "The feature you're looking for does not exist.".to_string(),
                )))
            }
            _ => return Err("Unknown argument."),
        }
    }
}

impl AppErr {
    pub fn js_default() -> AppErr {
        AppErr::Js(ErrMsg("Unexpected application error occured. Please ensure you are using the latest version of Firefox, Edge or Chrome.".to_string()))
    }

    pub fn gloo_default() -> AppErr {
        AppErr::Gloo(ErrMsg("Error in getting data from the server. This may occur because your network connection is unstable, or has been interrupted.".to_string()))
    }

    pub fn serde_default() -> AppErr {
        AppErr::Serde(ErrMsg("Error in formatting data. Usually this occurs because you have made too many requests in a short span of time. You will be redirected to your previous page soon.".to_string()))
    }

    pub fn not_found() -> AppErr {
        AppErr::NotFound(ErrMsg(
            "The feature you're looking for does not exist.".to_string(),
        ))
    }

    pub fn title(&self) -> String {
        match &self {
            AppErr::Gloo(_) => "Network error".to_string(),
            AppErr::Js(_) => "Javascript runtime error".to_string(),
            AppErr::Serde(_) => "Too many request".to_string(),
            AppErr::NotFound(_) => "404 Not Found".to_string(),
        }
    }

    pub fn msg(&self) -> String {
        match &self {
            AppErr::Gloo(ref msg) => msg.to_string(),
            AppErr::Js(ref msg) => msg.to_string(),
            AppErr::Serde(ref msg) => msg.to_string(),
            AppErr::NotFound(ref msg) => msg.to_string(),
        }
    }
}

impl Display for AppErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppErr::Gloo(ref msg) => {
                write!(f, "{}{}", "gl", msg.to_string())
            }
            AppErr::Js(ref msg) => {
                write!(f, "{}{}", "js", msg.to_string())
            }
            AppErr::Serde(ref msg) => {
                write!(f, "{}{}", "sd", msg.to_string())
            }
            AppErr::NotFound(ref msg) => {
                write!(f, "{}{}", "nf", msg.to_string())
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ErrMsg(pub String);

impl ErrMsg {
    pub fn to_string(&self) -> String {
        (&self).0.to_string()
    }
}

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub app_err: AppErr,
}

#[function_component]
pub fn ErrorPage(props: &ErrorProps) -> Html {
    let app_err = use_state(|| props.app_err.clone());
    let nav = use_navigator().unwrap();

    {
        let app_err = app_err.clone();
        let nav = nav.clone();
        use_effect_with((), move |_| {
            let app_err = app_err.clone();
            let nav = nav.clone();
            gloo::timers::callback::Timeout::new(4000, move || {
                let hist_length = web_sys::window()
                    .unwrap()
                    .history()
                    .unwrap()
                    .length()
                    .unwrap();
                if hist_length <= 2 {
                    nav.push(&Route::HomeNew)
                } else {
                    nav.back()
                }
            })
            .forget();
        });
    }

    html! {
        <>
        <div class="written-content">
        <h2 class="content-ttl">{"Error: "}{(*app_err).title()}</h2>
        <p>{(*app_err).msg()}</p>
        <p><b>{"You will automatically be redirected to the previous page or the home page."}</b></p>
        </div>
        </>
    }
}
