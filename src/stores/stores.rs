use std::default;

use crate::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use yew::prelude::*;
use yewdux::prelude::*;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum StdResultType {
    Top(&'static str),
    Seasonal(&'static str),
    Genre(u32),
    Producer(u32),
}

impl FromStr for StdResultType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() > 9 {
            let id = s[9..].parse::<u32>().expect("Invalid producer ID.");
            Ok(StdResultType::Producer(id))
        } else if s.chars().count() > 6 && s != "seasonal" {
            let id = s[6..].parse::<u32>().expect("Invalid genre ID.");
            Ok(StdResultType::Genre(id))
        } else {
            match s {
                "top" => Ok(TOP),
                "seasonal" => Ok(SEASONAL),
                _ => Err("Invalid StdResultType."),
            }
        }
    }
}

impl Display for StdResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match &self {
                &StdResultType::Seasonal(_) => "seasonal".to_string(),
                &StdResultType::Top(_) => "top".to_string(),
                &StdResultType::Genre(id) => format!("genre/{}", id),
                &StdResultType::Producer(id) => format!("producer/{}", id),
            }
        })
    }
}

impl StdResultType {
    pub fn link(&self) -> String {
        match &self {
            &StdResultType::Top(s) => s.to_string(),
            &StdResultType::Seasonal(s) => s.to_string(),
            &StdResultType::Genre(id) => {
                format!("{}?genres={}&order_by=score&sort=desc", JIKAN_URL, id)
            }
            &StdResultType::Producer(id) => {
                format!("{}?producers={}&order_by=score&sort=desc", JIKAN_URL, id)
            }
        }
    }
}

pub const TOP: StdResultType = StdResultType::Top("https://api.jikan.moe/v4/top/anime?");
pub const SEASONAL: StdResultType =
    StdResultType::Seasonal("https://api.jikan.moe/v4/seasons/now?");

#[derive(Store, Default, PartialEq, Debug, Clone)]
pub struct NavbarSearch {
    pub query: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct PaginationObj {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub current_page: u32,
    pub items: PaginationItems,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct PaginationItems {
    pub count: i32,
    pub total: i32,
    pub per_page: i32,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Images {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct AnimeImages {
    pub jpg: Images,
    pub webp: Images,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Trailers {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Title {
    pub r#type: String,
    pub title: String,
}

impl Title {
    pub fn get_type(&self) -> String {
        (&self.r#type).to_string()
    }
}

#[derive(PartialEq, Serialize, Deserialize, Default, Debug, Hash)]
pub struct TestObj {
    p1: u8,
    p2: u8,
    p3: u8,
    p4: u8,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct AnimeObj {
    pub mal_id: u64,
    pub url: String,
    pub images: AnimeImages,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u16>,
    #[serde(deserialize_with = "deserialize_status_from_option_string")]
    pub status: Option<AnimeStatus>,
    pub airing: bool,
    #[serde(deserialize_with = "deserialize_option_string_from_option_number")]
    pub score: Option<String>,
    pub scored_by: Option<u64>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub year: Option<u32>,
    pub r#type: Option<String>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub studios: Vec<MALObj>,
    pub genres: Vec<MALObj>,
    pub season: Option<String>,
    pub themes: Vec<MALObj>,
    pub aired: AiredDate,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Copy, Clone, Hash)]
pub enum AnimeStatus {
    FinishedAiring,
    CurrentlyAiring,
    NotYetAiring,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct AnimeObjFull {
    pub mal_id: u64,
    pub url: String,
    pub images: AnimeImages,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u16>,
    #[serde(deserialize_with = "deserialize_status_from_option_string")]
    pub status: Option<AnimeStatus>,
    pub airing: bool,
    #[serde(deserialize_with = "deserialize_option_string_from_option_number")]
    pub score: Option<String>,
    pub scored_by: Option<u64>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub year: Option<u32>,
    pub r#type: Option<String>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub studios: Vec<MALObj>,
    pub genres: Vec<MALObj>,
    pub season: Option<String>,
    pub themes: Vec<MALObj>,
    pub theme: AnimeTheme,
    pub aired: AiredDate,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct AiredDate {
    pub from: Option<String>,
}

impl From<AnimeObjFull> for AnimeObj {
    fn from(v: AnimeObjFull) -> Self {
        AnimeObj {
            mal_id: v.mal_id,
            url: v.url,
            images: v.images,
            approved: v.approved,
            titles: v.titles,
            title_english: v.title_english,
            title_japanese: v.title_japanese,
            source: v.source,
            episodes: v.episodes,
            status: v.status,
            airing: v.airing,
            score: v.score,
            scored_by: v.scored_by,
            rank: v.rank,
            popularity: v.popularity,
            year: v.year,
            r#type: v.r#type,
            synopsis: v.synopsis,
            background: v.background,
            studios: v.studios,
            genres: v.genres,
            season: v.season,
            themes: v.themes,
            aired: v.aired,
        }
    }
}

pub fn deserialize_option_string_from_option_number<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Float(f64),
        Null,
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(Some(s)),
        StringOrNumber::Number(i) => Ok(Some(i.to_string())),
        StringOrNumber::Float(f) => Ok(Some(f.to_string())),
        StringOrNumber::Null => Ok(None),
    }
}

pub fn deserialize_status_from_option_string<'de, D>(
    deserializer: D,
) -> Result<Option<AnimeStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNull {
        String(String),
        Null,
    }

    match StringOrNull::deserialize(deserializer)? {
        StringOrNull::String(s) => match s.as_str() {
            "Finished Airing" => Ok(Some(AnimeStatus::FinishedAiring)),
            "Currently Airing" => Ok(Some(AnimeStatus::CurrentlyAiring)),
            "Not yet aired" => Ok(Some(AnimeStatus::NotYetAiring)),
            _ => Err(serde::de::Error::custom("Invalid enum variants.")),
        },
        StringOrNull::Null => Ok(None),
    }
}

#[derive(Properties, Store, PartialEq, Serialize, Deserialize, Default, Debug, Clone, Hash)]
pub struct AnimeTheme {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct QueryResult {
    pub data: Vec<AnimeObj>,
    pub pagination: PaginationObj,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct AnimeObjAsProp {
    pub anime_obj: AnimeObj,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct AnimeObjAsQueryResult {
    pub data: AnimeObj,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct AnimeObjFullAsQueryResult {
    pub data: AnimeObjFull,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone, Hash)]
pub struct MALObj {
    pub mal_id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String,
}

#[derive(Properties, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct CharObj {
    pub mal_id: u32,
    pub url: String,
    pub images: CharImgWrapper,
    pub name: String,
}

impl CharObj {
    pub fn get_mal_id(&self) -> u32 {
        self.mal_id
    }

    pub fn get_url(&self) -> String {
        (&self).url.clone()
    }

    pub fn get_images_jpg(&self) -> Option<String> {
        if let Some(ref s) = (&self).images.jpg.image_url.as_ref() {
            Some(s.to_string())
        } else {
            None
        }
    }

    pub fn get_image_webp(&self) -> Option<String> {
        if let Some(ref s) = (&self).images.webp.image_url.as_ref() {
            Some(s.to_string())
        } else {
            None
        }
    }

    pub fn get_name(&self) -> String {
        (&self).name.clone()
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Char {
    pub character: CharObj,
    pub role: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct CharWrapper {
    pub data: Vec<Char>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct CharImgWrapper {
    pub jpg: CharImg,
    pub webp: CharImg,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct CharImg {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
}

pub type AnimeRecImg = CharImg;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct AnimeRecWrapper {
    pub entry: AnimeRecObj,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct AnimeRecResult {
    pub data: Vec<AnimeRecWrapper>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct AnimeRecObj {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub images: AnimeImages,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimeEpisode {
    pub mal_id: u32,
    pub url: Option<String>,
    pub title: String,
    pub title_japanese: Option<String>,
    pub title_romanji: Option<String>,
    pub aired: Option<String>,
    pub forum_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnimeEpisodeWrapper {
    pub data: Vec<AnimeEpisode>,
    pub pagination: AnimeEpisodePagination,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Hash)]
pub struct AnimeEpisodePagination {
    pub last_visible_page: u32,
    pub has_next_page: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SeasonObj {
    pub year: u32,
    pub seasons: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SeasonsWrapper {
    pub pagination: AnimeEpisodePagination,
    pub data: Vec<SeasonObj>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Hash)]
pub struct Cache {
    pub anime_details: Option<AnimeObjAsQueryResult>,
    pub search_result: Option<QueryResult>,
    pub anime_result: Option<QueryResult>,
    pub home_page_result: Option<(QueryResult, QueryResult)>,
}

impl Cache {
    pub fn update_anime_details(&self, anime_details: Option<AnimeObjAsQueryResult>) -> Cache {
        Cache {
            anime_details,
            search_result: (&self.search_result).clone(),
            anime_result: (&self.anime_result).clone(),
            home_page_result: (&self).home_page_result.clone(),
        }
    }

    pub fn update_anime_result(&self, anime_result: Option<QueryResult>) -> Cache {
        Cache {
            anime_details: (&self.anime_details).clone(),
            search_result: (&self.search_result).clone(),
            anime_result,
            home_page_result: (&self).home_page_result.clone(),
        }
    }

    pub fn update_search_result(&self, search_result: Option<QueryResult>) -> Cache {
        Cache {
            anime_details: (&self.anime_details).clone(),
            search_result,
            anime_result: (&self.anime_result).clone(),
            home_page_result: (&self).home_page_result.clone(),
        }
    }

    pub fn update_home_page_result(
        &self,
        home_page_result: Option<(QueryResult, QueryResult)>,
    ) -> Cache {
        Cache {
            anime_details: (&self.anime_details).clone(),
            search_result: (&self).search_result.clone(),
            anime_result: (&self.anime_result).clone(),
            home_page_result: home_page_result,
        }
    }
}

#[derive(Store, PartialEq, Default, Clone)]
pub struct NodeRefStore {
    pub nb_left: NodeRef,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Torrent {
    pub title: String,
    pub size_mb: String,
    pub link_magnet: String,
    pub link_torrent: String,
    pub link_view: String,
    pub download: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TorrentResponse {
    pub data: Vec<Torrent>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TorrentRequest {
    pub ttl_def: String,
    pub ttl_en: String,
    pub eps: u16,
    pub filter: Vec<Filter>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Filter {
    BDRip,
    HEVC,
    DDP,
    AMZN,
    FLAC,
    AllEpisodes
}

impl Filter {
    pub fn as_str(&self) -> &str {
        match self {
            Filter::BDRip => "BDRip",
            Filter::HEVC => "HEVC",
            Filter::AMZN => "AMZN",
            Filter::DDP => "DDP / AC3 / E-AC3",
            Filter::FLAC => "FLAC",
            Filter::AllEpisodes => "All Episodes"
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserRequest {
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserForResponse {
    pub username: String,
    pub uuid: i32,
    pub fav_anime: Vec<UserAnimeResponse>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeCommentGet {
    pub username: String,
    pub comment: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeCommentPost {
    pub comment: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Genre {
    pub mal_id: u32,
    pub name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Default, Copy)]
pub enum OrderBy {
    StartDate,
    #[default]
    Score,
    Rank,
}

impl OrderBy {
    pub fn to_str(&self) -> &'static str {
        match self {
            &OrderBy::StartDate => "start_date",
            &OrderBy::Score => "score",
            &OrderBy::Rank => "rank",
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Default, Copy)]
pub enum Sort {
    Asc,
    #[default]
    Desc,
}

impl Sort {
    pub fn to_str(&self) -> &'static str {
        match self {
            &Sort::Asc => "asc",
            &Sort::Desc => "desc",
        }
    }
}

#[derive(Store, PartialEq, Debug, Serialize, Deserialize, Clone, Default, Copy)]
pub struct QuerySort {
    pub sort: Sort,
    pub order_by: OrderBy,
}

impl QuerySort {
    pub fn to_params(&self) -> String {
        let mut url = "".to_string();
        match &self.sort {
            &Sort::Asc => url.push_str("&sort=asc"),
            &Sort::Desc => url.push_str("&sort=desc"),
        }
        match &self.order_by {
            &OrderBy::Rank => url.push_str("&order_by=rank"),
            &OrderBy::Score => url.push_str("&order_by=score"),
            &OrderBy::StartDate => url.push_str("&order_by=start_date"),
        }

        url
    }

    pub fn update_sort(&self, s: Sort) -> Self {
        Self {
            sort: s,
            order_by: self.order_by,
        }
    }

    pub fn update_order_by(&self, ord: OrderBy) -> Self {
        Self {
            sort: self.sort,
            order_by: ord,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserAnimeSubmission {
    pub uuid: i32,
    pub anime_id: i32,
    pub anime_img: Option<String>,
    pub anime_ttl_en: String,
    pub anime_ttl_jp: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserAnimeResponse {
    pub anime_id: i32,
    pub anime_img: Option<String>,
    pub anime_ttl_en: String,
    pub anime_ttl_jp: Option<String>
}



#[derive(Store, PartialEq, Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryFilter {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub genres: Vec<Genre>,
    pub genres_exclude: Vec<Genre>,
}

impl QueryFilter {
    pub fn to_params(&self) -> String {
        let mut url = "".to_string();

        if let Some(d) = self.start_date.as_ref() {
            url.push_str(&format!("&start_date={}", d));
        }

        if let Some(d) = self.end_date.as_ref() {
            url.push_str(&format!("&end_date={}", d));
        }

        if self.genres.len() != 0 {
            let mut genres = "&genres=".to_string();
            self.genres.iter().enumerate().for_each(|(i, g)| {
                if i + 1 != self.genres.len() {
                    genres.push_str(&format!("{},", g.mal_id));
                } else {
                    genres.push_str(&format!("{}", g.mal_id))
                }
            });
            url.push('&');
            url.push_str(&genres)
        }

        if self.genres_exclude.len() != 0 {
            let mut genres = "&genres_exclude=".to_string();
            self.genres_exclude.iter().enumerate().for_each(|(i, g)| {
                if i + 1 != self.genres.len() {
                    genres.push_str(&format!("{},", g.mal_id));
                } else {
                    genres.push_str(&format!("{}", g.mal_id))
                }
            });
            url.push('&');
            url.push_str(&genres);
        }
        url
    }

    pub fn add_genres(&self, g: &Genre) -> Self {
        let mut v = self.genres.clone();
        v.push((*g).clone());
        Self {
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            genres: v,
            genres_exclude: self.genres_exclude.clone(),
        }
    }

    pub fn reset_genres(&self) -> Self {
        Self {
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            genres: vec![],
            genres_exclude: self.genres_exclude.clone(),
        }
    }

    pub fn remove_genres(&self, g: &Genre) -> Self {
        let v = self
            .genres
            .clone()
            .into_iter()
            .filter(|g_v| g != g_v)
            .collect::<Vec<Genre>>();
        Self {
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            genres: v,
            genres_exclude: self.genres_exclude.clone(),
        }
    }

    pub fn add_genres_exclude(&self, g: &Genre) -> Self {
        let mut v = self.genres_exclude.clone();
        v.push((*g).clone());
        Self {
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            genres: self.genres.clone(),
            genres_exclude: v,
        }
    }

    pub fn reset_genres_exclude(&self) -> Self {
        Self {
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            genres: self.genres.clone(),
            genres_exclude: vec![],
        }
    }

    pub fn remove_genres_exclude(&self, g: &Genre) -> Self {
        let v = self
            .genres_exclude
            .clone()
            .into_iter()
            .filter(|g_v| g != g_v)
            .collect::<Vec<Genre>>();
        Self {
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            genres: self.genres.clone(),
            genres_exclude: v,
        }
    }

    pub fn add_start_year(&self, y: &str) -> Self {
        Self {
            start_date: Some(format!("{}-01-01", y)),
            end_date: self.end_date.clone(),
            genres: self.genres.clone(),
            genres_exclude: self.genres_exclude.clone(),
        }
    }

    pub fn remove_start_year(&self) -> Self {
        Self {
            start_date: None,
            end_date: self.end_date.clone(),
            genres: self.genres.clone(),
            genres_exclude: self.genres_exclude.clone(),
        }
    }

    pub fn add_end_year(&self, y: &str) -> Self {
        Self {
            end_date: Some(format!("{}-12-31", y)),
            start_date: self.start_date.clone(),
            genres: self.genres.clone(),
            genres_exclude: self.genres_exclude.clone(),
        }
    }

    pub fn remove_end_year(&self) -> Self {
        Self {
            end_date: None,
            start_date: self.start_date.clone(),
            genres: self.genres.clone(),
            genres_exclude: self.genres_exclude.clone(),
        }
    }
}

#[derive(Store, PartialEq, Clone)]
pub struct ExplorePage(pub u32);

impl Default for ExplorePage {
    fn default() -> Self {
        Self(1)
    }
}

pub fn binary_search<T: PartialOrd + Debug>(v: &[T], t: &T) -> Option<usize> {
    // Immediately return if length of v is 0
    if v.len() == 0 {
        return None;
    }
    
    println!("Finding for {:?} in {:?}", t, v);
    
    // Immediately return if target is smaller than first element or larger
    // than last element
    if t < &v[0] || t > &v[v.len()-1] {
        return None;
    }
    
    let mut left = 0usize;
    let mut right = v.len() - 1;
    let mut mid = (left + right) / 2;
    
    // Immediately return if mid is target
    if &v[mid] == t && mid > 0 {
        while &v[mid] == t && mid > 0 {
            mid -= 1;
        }
        if mid == 0 && &v[mid] == t {
            return Some(mid);
        } else {
          return Some(mid+1);  
        }
    } else if &v[mid] == t {
        return Some(mid);
    }
    
    // While the window length is at least 2, continue comparing with mid
    // and recalculating mid until found or window length is 1
    while left <= right {
        if &v[mid] == t && mid > 0 {
            while &v[mid] == t && mid > 0 {
                mid -= 1;
            }
            if mid == 0 && &v[mid] == t {
                return Some(mid);
            } else {
              return Some(mid+1);  
            }
        } else if &v[mid] == t {
            return Some(mid);
        } else if &v[mid] < t {
            left = mid + 1;
            mid = (left + right) / 2;
        } else if &v[mid] > t {
            if mid >= 1 {
                right = mid - 1;
                mid = (left + right) / 2;
            } else {
                return None;
            }
        }
    }
    
    // If there is only one element in the current window, then compare with it
    // if left == right && &v[mid] == t {
    //     return Some(mid)
    // }

    return None
}

// Helper function to compare Option<usize> for testing
fn assert_option_usize_eq(result: Option<usize>, expected: Option<usize>) {
    match (result, expected) {
        (None, None) => (),
        (Some(r), Some(e)) => assert_eq!(r, e),
        _ => panic!("Expected {:?}, got {:?}", expected, result),
    }
}

fn main() {
    let x = [50000, 60000];
    let target = 1;
    let target_i = binary_search(&x, &target);
    match target_i {
        None => println!("{} not found in array 'x'.", &target),
        Some(i) => println!("{} found in array 'x' at index {i}", &target)
    }
    println!("Hello, world!");
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_binary_search() {
        // Test cases with an empty array
        let array_empty: &[i32] = &[];
        assert_option_usize_eq(binary_search(array_empty, &10), None);

        // Test cases with single-element arrays
        let array_single = &[5];
        assert_option_usize_eq(binary_search(array_single, &5), Some(0));
        assert_option_usize_eq(binary_search(array_single, &10), None);

        // Test cases with multiple-element arrays
        let array_multiple = &[2, 4, 6, 8, 10];
        assert_option_usize_eq(binary_search(array_multiple, &4), Some(1));
        assert_option_usize_eq(binary_search(array_multiple, &2), Some(0));
        assert_option_usize_eq(binary_search(array_multiple, &10), Some(4));
        assert_option_usize_eq(binary_search(array_multiple, &5), None);
        assert_option_usize_eq(binary_search(array_multiple, &1), None);
        assert_option_usize_eq(binary_search(array_multiple, &11), None);

        // Test cases with repeated elements
        let array_repeated = &[1, 2, 2, 2, 3, 4, 5];
        assert_option_usize_eq(binary_search(array_repeated, &2), Some(1));
        assert_option_usize_eq(binary_search(array_repeated, &5), Some(6));
        assert_option_usize_eq(binary_search(array_repeated, &0), None);
        assert_option_usize_eq(binary_search(array_repeated, &6), None);

        // Test cases with negative numbers
        let array_negative = &[-10, -5, 0, 5, 10];
        assert_option_usize_eq(binary_search(array_negative, &0), Some(2));
        assert_option_usize_eq(binary_search(array_negative, &-10), Some(0));
        assert_option_usize_eq(binary_search(array_negative, &10), Some(4));
        assert_option_usize_eq(binary_search(array_negative, &-6), None);
        assert_option_usize_eq(binary_search(array_negative, &6), None);

        // Additional tests for edge cases
        let array_edge = &[1, 3, 5, 7, 9];
        assert_option_usize_eq(binary_search(array_edge, &1), Some(0));
        assert_option_usize_eq(binary_search(array_edge, &9), Some(4));
        assert_option_usize_eq(binary_search(array_edge, &4), None);
        assert_option_usize_eq(binary_search(array_edge, &10), None);
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_binary_search() {
        // Test cases with an empty array
        let array_empty: &[i32] = &[];
        assert_option_usize_eq(binary_search(array_empty, &10), None);

        // Test cases with single-element arrays
        let array_single = &[5];
        assert_option_usize_eq(binary_search(array_single, &5), Some(0));
        assert_option_usize_eq(binary_search(array_single, &10), None);

        // Test cases with multiple-element arrays
        let array_multiple = &[2, 4, 6, 8, 10];
        assert_option_usize_eq(binary_search(array_multiple, &4), Some(1));
        assert_option_usize_eq(binary_search(array_multiple, &2), Some(0));
        assert_option_usize_eq(binary_search(array_multiple, &10), Some(4));
        assert_option_usize_eq(binary_search(array_multiple, &5), None);
        assert_option_usize_eq(binary_search(array_multiple, &1), None);
        assert_option_usize_eq(binary_search(array_multiple, &11), None);

        // Test cases with repeated elements
        let array_repeated = &[1, 2, 2, 2, 3, 4, 5];
        assert_option_usize_eq(binary_search(array_repeated, &2), Some(1));
        assert_option_usize_eq(binary_search(array_repeated, &5), Some(6));
        assert_option_usize_eq(binary_search(array_repeated, &0), None);
        assert_option_usize_eq(binary_search(array_repeated, &6), None);

        // Test cases with negative numbers
        let array_negative = &[-10, -5, 0, 5, 10];
        assert_option_usize_eq(binary_search(array_negative, &0), Some(2));
        assert_option_usize_eq(binary_search(array_negative, &-10), Some(0));
        assert_option_usize_eq(binary_search(array_negative, &10), Some(4));
        assert_option_usize_eq(binary_search(array_negative, &-6), None);
        assert_option_usize_eq(binary_search(array_negative, &6), None);

        // Additional tests for edge cases
        let array_edge = &[1, 3, 5, 7, 9];
        assert_option_usize_eq(binary_search(array_edge, &1), Some(0));
        assert_option_usize_eq(binary_search(array_edge, &9), Some(4));
        assert_option_usize_eq(binary_search(array_edge, &4), None);
        assert_option_usize_eq(binary_search(array_edge, &10), None);

        // More test cases to ensure first occurrence with duplicates
        let array_duplicates = &[1, 2, 2, 2, 3, 4, 4, 5, 5, 5];
        assert_option_usize_eq(binary_search(array_duplicates, &2), Some(1));
        assert_option_usize_eq(binary_search(array_duplicates, &4), Some(5));
        assert_option_usize_eq(binary_search(array_duplicates, &5), Some(7));
        assert_option_usize_eq(binary_search(array_duplicates, &6), None);
        assert_option_usize_eq(binary_search(array_duplicates, &0), None);

        // Generating more tests with various patterns and sizes...

        // Continue adding more diverse test cases as needed
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn test_binary_search() {
        // Test cases with an empty array
        let array_empty: &[i32] = &[];
        assert_option_usize_eq(binary_search(array_empty, &10), None);

        // Test cases with single-element arrays
        let array_single = &[5];
        assert_option_usize_eq(binary_search(array_single, &5), Some(0));
        assert_option_usize_eq(binary_search(array_single, &10), None);

        // Test cases with multiple-element arrays
        let array_multiple = &[2, 4, 6, 8, 10];
        assert_option_usize_eq(binary_search(array_multiple, &4), Some(1));
        assert_option_usize_eq(binary_search(array_multiple, &2), Some(0));
        assert_option_usize_eq(binary_search(array_multiple, &10), Some(4));
        assert_option_usize_eq(binary_search(array_multiple, &5), None);
        assert_option_usize_eq(binary_search(array_multiple, &1), None);
        assert_option_usize_eq(binary_search(array_multiple, &11), None);

        // Test cases with repeated elements
        let array_repeated = &[1, 2, 2, 2, 3, 4, 5];
        assert_option_usize_eq(binary_search(array_repeated, &2), Some(1));
        assert_option_usize_eq(binary_search(array_repeated, &5), Some(6));
        assert_option_usize_eq(binary_search(array_repeated, &0), None);
        assert_option_usize_eq(binary_search(array_repeated, &6), None);

        // Test cases with negative numbers
        let array_negative = &[-10, -5, 0, 5, 10];
        assert_option_usize_eq(binary_search(array_negative, &0), Some(2));
        assert_option_usize_eq(binary_search(array_negative, &-10), Some(0));
        assert_option_usize_eq(binary_search(array_negative, &10), Some(4));
        assert_option_usize_eq(binary_search(array_negative, &-6), None);
        assert_option_usize_eq(binary_search(array_negative, &6), None);

        // Additional tests for edge cases
        let array_edge = &[1, 3, 5, 7, 9];
        assert_option_usize_eq(binary_search(array_edge, &1), Some(0));
        assert_option_usize_eq(binary_search(array_edge, &9), Some(4));
        assert_option_usize_eq(binary_search(array_edge, &4), None);
        assert_option_usize_eq(binary_search(array_edge, &10), None);

        // More test cases with diverse patterns
        let array_large = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        assert_option_usize_eq(binary_search(array_large, &15), Some(14));
        assert_option_usize_eq(binary_search(array_large, &1), Some(0));
        assert_option_usize_eq(binary_search(array_large, &20), Some(19));
        assert_option_usize_eq(binary_search(array_large, &0), None);
        assert_option_usize_eq(binary_search(array_large, &21), None);

        let array_odd_length = &[1, 3, 5, 7, 9, 11];
        assert_option_usize_eq(binary_search(array_odd_length, &5), Some(2));
        assert_option_usize_eq(binary_search(array_odd_length, &1), Some(0));
        assert_option_usize_eq(binary_search(array_odd_length, &11), Some(5));
        assert_option_usize_eq(binary_search(array_odd_length, &6), None);
        assert_option_usize_eq(binary_search(array_odd_length, &0), None);

        let array_alternating = &[2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        assert_option_usize_eq(binary_search(array_alternating, &6), Some(2));
        assert_option_usize_eq(binary_search(array_alternating, &10), Some(4));
        assert_option_usize_eq(binary_search(array_alternating, &20), Some(9));
        assert_option_usize_eq(binary_search(array_alternating, &1), None);
        assert_option_usize_eq(binary_search(array_alternating, &21), None);

        // Additional edge cases
        let array_large_negative = &[-100, -50, -25, 0, 25, 50, 75, 100];
        assert_option_usize_eq(binary_search(array_large_negative, &0), Some(3));
        assert_option_usize_eq(binary_search(array_large_negative, &-100), Some(0));
        assert_option_usize_eq(binary_search(array_large_negative, &100), Some(7));
        assert_option_usize_eq(binary_search(array_large_negative, &75), Some(6));
        assert_option_usize_eq(binary_search(array_large_negative, &101), None);

        let array_all_same = &[5; 10];
        assert_option_usize_eq(binary_search(array_all_same, &5), Some(0));
        assert_option_usize_eq(binary_search(array_all_same, &6), None);

        let array_two_elements = &[3, 7];
        assert_option_usize_eq(binary_search(array_two_elements, &3), Some(0));
        assert_option_usize_eq(binary_search(array_two_elements, &7), Some(1));
        assert_option_usize_eq(binary_search(array_two_elements, &5), None);

        let array_single_negative = &[-3];
        assert_option_usize_eq(binary_search(array_single_negative, &-3), Some(0));
        assert_option_usize_eq(binary_search(array_single_negative, &0), None);

        let array_large_odd = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
        assert_option_usize_eq(binary_search(array_large_odd, &10), Some(9));
        assert_option_usize_eq(binary_search(array_large_odd, &1), Some(0));
        assert_option_usize_eq(binary_search(array_large_odd, &19), Some(18));
        assert_option_usize_eq(binary_search(array_large_odd, &0), None);
        assert_option_usize_eq(binary_search(array_large_odd, &20), None);

        // Continue adding more diverse test cases as needed
    }
}