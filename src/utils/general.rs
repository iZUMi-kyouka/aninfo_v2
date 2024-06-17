use gloo::console::log;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub fn handle_season(month: &str) -> &'static str {
    let month = month.parse::<u8>().unwrap();
    if month == 1 || month == 2 || month == 3 {
        "Winter"
    } else if month == 4 || month == 5 || month == 6 {
        "Spring"
    } else if month == 7 || month == 8 || month == 9 {
        "Summer"
    } else if month == 10 || month == 11 || month == 12 {
        "Fall"
    } else {
        panic!("Invalid season.")
    }
}

pub async fn fetch_data_into<T: Serialize + DeserializeOwned + Debug>(
    url: &str,
) -> Result<T, gloo::net::Error> {
    gloo::net::http::Request::get(url)
        .send()
        .await?
        .json::<T>()
        .await
}

pub async fn force_req<T: Serialize + DeserializeOwned + Debug>(
    url: &str,
) -> Result<T, gloo::net::Error> {
    let mut count = 1;
    let mut fetched_data = fetch_data_into::<T>(url).await;

    while let &Err(ref e) = &fetched_data {
        log!(&e.to_string());
        fetched_data = fetch_data_into::<T>(url).await;
        gloo::timers::future::TimeoutFuture::new(100).await;
        log!("ERROR OCCURRED IN FETCHING DATA. RETRYING...");

        count += 1;
        if count >= 20 {
            return Err(gloo::net::Error::GlooError("API error. Please retry again soon.".to_string()));
        }
    }

    fetched_data
}

pub async fn force_req_with_body<
    T: Serialize + DeserializeOwned + Debug,
    V: Serialize + DeserializeOwned + Debug + Clone,
>(
    url: &str,
    body: V,
) -> Result<T, gloo::net::Error> {
    let mut fetched_data = fetch_data_into_with_body::<T, V>(url, body.clone()).await;

    while let &Err(_) = &fetched_data {
        log!("error fetching data occurred. retrying in 1000ms.");
        fetched_data = fetch_data_into_with_body::<T, V>(url, body.clone()).await;
        gloo::timers::future::TimeoutFuture::new(100).await;
    }

    fetched_data
}

pub async fn fetch_data_into_with_body<
    T: Serialize + DeserializeOwned + Debug,
    V: Serialize + DeserializeOwned + Debug + Clone,
>(
    url: &str,
    body: V,
) -> Result<T, gloo::net::Error> {
    gloo::net::http::Request::post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())?
        .send()
        .await?
        .json::<T>()
        .await
}

pub async fn force_req_text(url: &str) -> Result<String, gloo::net::Error> {
    let mut fetched_data = reqwasm::http::Request::get(url)
        .header("Access-Control-Allow-Origin", "*")
        .send()
        .await;

    while let &Err(_) = &fetched_data {
        fetched_data = reqwasm::http::Request::get(url).send().await;
        gloo::timers::future::TimeoutFuture::new(100).await;
    }

    Ok(fetched_data.unwrap().text().await.unwrap())
}

pub async fn force_req_text_with_body<V: DeserializeOwned + Serialize + Debug + Clone>(
    url: &str,
    body: V,
) -> Result<String, gloo::net::Error> {
    let mut count = 1;
    log!(format!("{}", serde_json::to_string_pretty(&body).unwrap()));
    let mut fetched_data = reqwasm::http::Request::post(url)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(
            serde_json::to_string(&body)
                .map_err(|_| gloo::net::Error::GlooError("JSON Parsing Error.".to_string()))
                .unwrap(),
        )
        .send()
        .await;

    while let &Err(_) = &fetched_data {
        // log!(e.as_string());
        log!("Error fetching from aninfo-server");
        log!(format!("{}", serde_json::to_string_pretty(&body).unwrap()));
        fetched_data = reqwasm::http::Request::get(url)
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(&body)
                    .map_err(|_| gloo::net::Error::GlooError("JSON Parsing Error.".to_string()))
                    .unwrap(),
            )
            .send()
            .await;
        gloo::timers::future::TimeoutFuture::new(100).await;
        count += 1;
        if count >= 20 {
            return Ok("Error".to_string());
        }
    }

    Ok(fetched_data.unwrap().text().await.unwrap())
}

use crate::Filter;
use crate::Torrent;
use crate::TorrentRequest;
use crate::BASE_URL;

#[derive(Debug)]
pub struct ThemesongParseResult {
    pub artist: String,
    pub title: String,
    pub eps: Option<String>,
}

pub fn parse_themesong(s: &str) -> Result<ThemesongParseResult, &'static str> {
    let e_i = s.find("(eps").ok_or("Invalid token.");
    let t_i = s.find("\"").ok_or("Invalid token.")?;
    let a_i = s.find("by ").ok_or("Invalid token.")?;
    Ok(ThemesongParseResult {
        artist: s[a_i + 3..{
            if let Ok(n) = e_i {
                n - 1
            } else {
                s.len()
            }
        }]
            .to_string(),
        title: s[t_i + 1..a_i - 2].to_string(),
        eps: {
            if let Ok(n) = e_i {
                Some(s[n + 5..s.len() - 1].to_string())
            } else {
                None
            }
        },
    })
}

pub async fn get_torrents(
    ttl_en: &str,
    ttl_def: &str,
    eps: u16,
    f: &[Filter],
    full: bool,
) -> Vec<Torrent> {
    let req = TorrentRequest {
        ttl_def: ttl_def.to_string(),
        ttl_en: ttl_en.to_string(),
        eps,
        filter: Vec::from(f),
    };
    if full || f.contains(&Filter::AllEpisodes) {
        let url = format!("{}/api/v1/get_torrent_full", BASE_URL);
        let html = force_req_with_body::<Vec<Torrent>, TorrentRequest>(&url, req)
            .await
            .unwrap();
        html
    } else {
        let url = format!("{}/api/v1/get_torrent", BASE_URL);
        let html = force_req_with_body::<Vec<Torrent>, TorrentRequest>(&url, req)
            .await
            .unwrap();
        html
    }
}

// fn handle_filter(f: &[Filter]) -> String {
//     if f.len() == 0 {
//         "".to_string()
//     } else {
//         let mut res = "".to_string();
//         f.iter().for_each(|f: &Filter| {
//             match f {
//                 &Filter::BDRip => res.push_str("BDRip"),
//                 &Filter::FLAC => res.push_str("FLAC")
//             }

//             res.push(' ');
//         });

//         res
//     }
// }

pub fn verify_password(p1: &str, p2: &str) -> bool {
    if p1 != p2 {
        return false;
    } else if p1.len() < 8 {
        return false;
    } else {
        let mut n = 0;
        let mut l = 0;
        let mut s = 0;
        let mut u = 0;
        for c in p1.chars() {
            if (!c.is_ascii_lowercase())
                && (!c.is_ascii_uppercase())
                && (!c.is_ascii_digit())
                && (!c.is_ascii_punctuation())
            {
                return false;
            } else if c.is_lowercase() {
                l += 1;
            } else if c.is_numeric() {
                n += 1;
            } else if c.is_uppercase() {
                u += 1;
            } else if c.is_ascii_punctuation() {
                s += 1
            }
        }

        return n > 0 && l > 0 && s > 0 && u > 0;
    }
}

pub fn cur_year() -> u32 {
    let dt = chrono::Utc::now();
    let s = format!("{}", dt.format("%Y"));
    s.parse::<u32>().unwrap()
}
