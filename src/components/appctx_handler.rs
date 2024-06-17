use crate::prelude::*;
use std::rc::Rc;

use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Hash, Copy)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    Auto,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Hash, Copy)]
pub enum Language {
    #[default]
    EN,
    JP,
}

pub const LIGHT_THEME: Theme = Theme::Light;
pub const DARK_THEME: Theme = Theme::Dark;
pub const AUTO_THEME: Theme = Theme::Auto;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct AppCtx {
    pub jwt: Option<String>,
    pub username: Option<String>,
    pub fav_anime: Option<Vec<UserAnimeResponse>>,
    pub fav_anime_id: Option<Vec<i32>>,
    pub uuid: Option<i32>,
    pub theme: Theme,
    pub language: Language,
    pub cur_page: u8,
    pub loading_page: bool,
    pub nsfw: bool,
    pub query: String,
    pub cache: Cache,
    hash: u64,
}

impl Reducible for AppCtx {
    type Action = AppCtx;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}

impl AppCtx {
    pub fn update_theme_into(&self, new_theme: Theme) -> AppCtx {
        AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: new_theme,
            language: (&self.language).clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (*self).query.clone(),
            cache: (&self.cache).clone(),
            hash: (&self).hash,
            ..self.clone()
        }
    }

    pub fn update_language_into(&self, new_lang: Language) -> AppCtx {
        AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: new_lang,
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (*self).query.clone(),
            cache: (&self.cache).clone(),
            hash: (&self).hash,
            ..self.clone()
        }
    }

    pub fn update_page_into(&self, new_page: u8) -> AppCtx {
        let ctx = AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: new_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (*self).query.clone(),
            cache: (&self.cache).clone(),
            hash: (&self).hash,
            ..self.clone()
        };
        ctx
    }

    pub fn update_loading_page_into(&self, new_status: bool) -> AppCtx {
        AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: new_status,
            nsfw: (&self).nsfw,
            query: (*self).query.clone(),
            cache: (&self.cache).clone(),
            hash: (&self).hash,
            ..self.clone()
        }
    }

    pub fn update_nsfw_into(&self, new_status: bool) -> AppCtx {
        let ctx = AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: new_status,
            query: (*self).query.clone(),
            cache: (&self.cache).clone(),
            hash: (&self).hash,
            ..self.clone()
        };
        ctx
    }

    pub fn update_query_into(&self, query: String) -> AppCtx {
        let ctx = AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query,
            cache: (&self.cache).clone(),
            hash: (&self).hash,
            ..self.clone()
        };
        ctx
    }

    pub fn update_cache_into(&self, cache: Cache) -> AppCtx {
        let ctx = AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (&self.query).clone(),
            cache,
            hash: (&self).hash,
            ..self.clone()
        };

        ctx.update_hash_into(calculate_hash(&ctx))
    }

    pub fn update_hash_into(&self, hash: u64) -> AppCtx {
        AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (&self.query).clone(),
            cache: (&self).cache.clone(),
            hash,
            ..self.clone()
        }
    }

    pub fn update_jwt_into(&self, jwt: Option<String>) -> AppCtx {
        AppCtx {
            uuid: (&self.uuid).clone(),
            jwt,
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (&self.query).clone(),
            cache: (&self).cache.clone(),
            hash: (&self).hash,
            ..(self.clone())
        }
    }
    pub fn update_username_into(&self, username: Option<String>) -> AppCtx {
        AppCtx {
            uuid: (&self.uuid).clone(),
            jwt: (&self.jwt).clone(),
            username,
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (&self.query).clone(),
            cache: (&self).cache.clone(),
            hash: (&self).hash,
            ..(self.clone())
        }
    }

    pub fn update_uuid_into(&self, uuid: Option<i32>) -> AppCtx {
        AppCtx {
            uuid: uuid,
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (&self.query).clone(),
            cache: (&self).cache.clone(),
            hash: (&self).hash,
            ..(self.clone())
        }
    }

    pub fn update_fav_anime_into(&self, fav_anime: Option<Vec<UserAnimeResponse>>) -> AppCtx {
        AppCtx {
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (&self.query).clone(),
            cache: (&self).cache.clone(),
            hash: (&self).hash,
            fav_anime,
            ..self.clone()
        }
    }

    pub fn update_fav_anime_id_into(&self, fav_anime_id: Option<Vec<i32>>) -> AppCtx {
        AppCtx {
            jwt: (&self.jwt).clone(),
            username: (&self.username).clone(),
            theme: (&self.theme).clone(),
            language: (&self).language.clone(),
            cur_page: (&self).cur_page,
            loading_page: (&self).loading_page,
            nsfw: (&self).nsfw,
            query: (&self.query).clone(),
            cache: (&self).cache.clone(),
            hash: (&self).hash,
            fav_anime_id,
            ..self.clone()
        }
    }

    pub fn get_langauge(&self) -> Language {
        (self.language).clone()
    }

    pub fn has_changed(&self) -> (bool, Option<AppCtx>) {
        let cur_hash = calculate_hash(self);
        let result = !(self.hash == cur_hash);
        if result {
            (true, Some(self.update_hash_into(cur_hash)))
        } else {
            (false, None)
        }
    }
}

impl Hash for AppCtx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cur_page.hash(state);
        self.nsfw.hash(state);
        self.query.hash(state);
    }
}

pub type AppContext = UseReducerHandle<AppCtx>;

#[derive(Properties, Debug, PartialEq)]
pub struct AppContextProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(AppContextProvider)]
pub fn app(props: &AppContextProviderProps) -> Html {
    let init_ctx = AppCtx {
        uuid: None,
        jwt: None,
        fav_anime: None,
        fav_anime_id: None,
        username: None,
        theme: DARK_THEME,
        language: Language::EN,
        cur_page: 1,
        loading_page: false,
        nsfw: false,
        query: "".to_string(),
        cache: Cache {
            anime_details: None,
            search_result: None,
            anime_result: None,
            home_page_result: None,
        },
        hash: 0,
    };

    let init_ctx = init_ctx.update_hash_into(calculate_hash(&init_ctx));

    let app_ctx = use_reducer(|| init_ctx);

    let app_ctx_cloned = app_ctx.clone();

    html! {
        <ContextProvider<AppContext> context={app_ctx}>
            <div class={format!("wrapper {}", handle_theme(&app_ctx_cloned))}>
            {props.children.clone()}
            </div>
        </ContextProvider<AppContext>>
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
