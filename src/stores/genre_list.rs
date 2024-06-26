use crate::prelude::*;

pub const GENRES: &'static str = r#"[
      {
        "mal_id": 1,
        "name": "Action",
        "url": "https://myanimelist.net/anime/genre/1/Action",
        "count": 5089
      },
      {
        "mal_id": 2,
        "name": "Adventure",
        "url": "https://myanimelist.net/anime/genre/2/Adventure",
        "count": 4037
      },
      {
        "mal_id": 5,
        "name": "Avant Garde",
        "url": "https://myanimelist.net/anime/genre/5/Avant_Garde",
        "count": 892
      },
      {
        "mal_id": 46,
        "name": "Award Winning",
        "url": "https://myanimelist.net/anime/genre/46/Award_Winning",
        "count": 248
      },
      {
        "mal_id": 28,
        "name": "Boys Love",
        "url": "https://myanimelist.net/anime/genre/28/Boys_Love",
        "count": 178
      },
      {
        "mal_id": 4,
        "name": "Comedy",
        "url": "https://myanimelist.net/anime/genre/4/Comedy",
        "count": 7283
      },
      {
        "mal_id": 8,
        "name": "Drama",
        "url": "https://myanimelist.net/anime/genre/8/Drama",
        "count": 2911
      },
      {
        "mal_id": 10,
        "name": "Fantasy",
        "url": "https://myanimelist.net/anime/genre/10/Fantasy",
        "count": 5724
      },
      {
        "mal_id": 26,
        "name": "Girls Love",
        "url": "https://myanimelist.net/anime/genre/26/Girls_Love",
        "count": 109
      },
      {
        "mal_id": 47,
        "name": "Gourmet",
        "url": "https://myanimelist.net/anime/genre/47/Gourmet",
        "count": 175
      },
      {
        "mal_id": 14,
        "name": "Horror",
        "url": "https://myanimelist.net/anime/genre/14/Horror",
        "count": 556
      },
      {
        "mal_id": 7,
        "name": "Mystery",
        "url": "https://myanimelist.net/anime/genre/7/Mystery",
        "count": 916
      },
      {
        "mal_id": 22,
        "name": "Romance",
        "url": "https://myanimelist.net/anime/genre/22/Romance",
        "count": 2008
      },
      {
        "mal_id": 24,
        "name": "Sci-Fi",
        "url": "https://myanimelist.net/anime/genre/24/Sci-Fi",
        "count": 3252
      },
      {
        "mal_id": 36,
        "name": "Slice of Life",
        "url": "https://myanimelist.net/anime/genre/36/Slice_of_Life",
        "count": 1648
      },
      {
        "mal_id": 30,
        "name": "Sports",
        "url": "https://myanimelist.net/anime/genre/30/Sports",
        "count": 765
      },
      {
        "mal_id": 37,
        "name": "Supernatural",
        "url": "https://myanimelist.net/anime/genre/37/Supernatural",
        "count": 1508
      },
      {
        "mal_id": 41,
        "name": "Suspense",
        "url": "https://myanimelist.net/anime/genre/41/Suspense",
        "count": 385
      },
      {
        "mal_id": 9,
        "name": "Ecchi",
        "url": "https://myanimelist.net/anime/genre/9/Ecchi",
        "count": 804
      },
      {
        "mal_id": 49,
        "name": "Erotica",
        "url": "https://myanimelist.net/anime/genre/49/Erotica",
        "count": 73
      },
      {
        "mal_id": 12,
        "name": "Hentai",
        "url": "https://myanimelist.net/anime/genre/12/Hentai",
        "count": 1519
      },
      {
        "mal_id": 50,
        "name": "Adult Cast",
        "url": "https://myanimelist.net/anime/genre/50/Adult_Cast",
        "count": 604
      },
      {
        "mal_id": 51,
        "name": "Anthropomorphic",
        "url": "https://myanimelist.net/anime/genre/51/Anthropomorphic",
        "count": 980
      },
      {
        "mal_id": 52,
        "name": "CGDCT",
        "url": "https://myanimelist.net/anime/genre/52/CGDCT",
        "count": 244
      },
      {
        "mal_id": 53,
        "name": "Childcare",
        "url": "https://myanimelist.net/anime/genre/53/Childcare",
        "count": 69
      },
      {
        "mal_id": 54,
        "name": "Combat Sports",
        "url": "https://myanimelist.net/anime/genre/54/Combat_Sports",
        "count": 91
      },
      {
        "mal_id": 81,
        "name": "Crossdressing",
        "url": "https://myanimelist.net/anime/genre/81/Crossdressing",
        "count": 50
      },
      {
        "mal_id": 55,
        "name": "Delinquents",
        "url": "https://myanimelist.net/anime/genre/55/Delinquents",
        "count": 73
      },
      {
        "mal_id": 39,
        "name": "Detective",
        "url": "https://myanimelist.net/anime/genre/39/Detective",
        "count": 302
      },
      {
        "mal_id": 56,
        "name": "Educational",
        "url": "https://myanimelist.net/anime/genre/56/Educational",
        "count": 278
      },
      {
        "mal_id": 57,
        "name": "Gag Humor",
        "url": "https://myanimelist.net/anime/genre/57/Gag_Humor",
        "count": 245
      },
      {
        "mal_id": 58,
        "name": "Gore",
        "url": "https://myanimelist.net/anime/genre/58/Gore",
        "count": 258
      },
      {
        "mal_id": 35,
        "name": "Harem",
        "url": "https://myanimelist.net/anime/genre/35/Harem",
        "count": 470
      },
      {
        "mal_id": 59,
        "name": "High Stakes Game",
        "url": "https://myanimelist.net/anime/genre/59/High_Stakes_Game",
        "count": 50
      },
      {
        "mal_id": 13,
        "name": "Historical",
        "url": "https://myanimelist.net/anime/genre/13/Historical",
        "count": 1560
      },
      {
        "mal_id": 60,
        "name": "Idols (Female)",
        "url": "https://myanimelist.net/anime/genre/60/Idols_Female",
        "count": 320
      },
      {
        "mal_id": 61,
        "name": "Idols (Male)",
        "url": "https://myanimelist.net/anime/genre/61/Idols_Male",
        "count": 176
      },
      {
        "mal_id": 62,
        "name": "Isekai",
        "url": "https://myanimelist.net/anime/genre/62/Isekai",
        "count": 361
      },
      {
        "mal_id": 63,
        "name": "Iyashikei",
        "url": "https://myanimelist.net/anime/genre/63/Iyashikei",
        "count": 171
      },
      {
        "mal_id": 64,
        "name": "Love Polygon",
        "url": "https://myanimelist.net/anime/genre/64/Love_Polygon",
        "count": 97
      },
      {
        "mal_id": 65,
        "name": "Magical Sex Shift",
        "url": "https://myanimelist.net/anime/genre/65/Magical_Sex_Shift",
        "count": 30
      },
      {
        "mal_id": 66,
        "name": "Mahou Shoujo",
        "url": "https://myanimelist.net/anime/genre/66/Mahou_Shoujo",
        "count": 337
      },
      {
        "mal_id": 17,
        "name": "Martial Arts",
        "url": "https://myanimelist.net/anime/genre/17/Martial_Arts",
        "count": 620
      },
      {
        "mal_id": 18,
        "name": "Mecha",
        "url": "https://myanimelist.net/anime/genre/18/Mecha",
        "count": 1264
      },
      {
        "mal_id": 67,
        "name": "Medical",
        "url": "https://myanimelist.net/anime/genre/67/Medical",
        "count": 49
      },
      {
        "mal_id": 38,
        "name": "Military",
        "url": "https://myanimelist.net/anime/genre/38/Military",
        "count": 709
      },
      {
        "mal_id": 19,
        "name": "Music",
        "url": "https://myanimelist.net/anime/genre/19/Music",
        "count": 4424
      },
      {
        "mal_id": 6,
        "name": "Mythology",
        "url": "https://myanimelist.net/anime/genre/6/Mythology",
        "count": 652
      },
      {
        "mal_id": 68,
        "name": "Organized Crime",
        "url": "https://myanimelist.net/anime/genre/68/Organized_Crime",
        "count": 94
      },
      {
        "mal_id": 69,
        "name": "Otaku Culture",
        "url": "https://myanimelist.net/anime/genre/69/Otaku_Culture",
        "count": 103
      },
      {
        "mal_id": 20,
        "name": "Parody",
        "url": "https://myanimelist.net/anime/genre/20/Parody",
        "count": 767
      },
      {
        "mal_id": 70,
        "name": "Performing Arts",
        "url": "https://myanimelist.net/anime/genre/70/Performing_Arts",
        "count": 117
      },
      {
        "mal_id": 71,
        "name": "Pets",
        "url": "https://myanimelist.net/anime/genre/71/Pets",
        "count": 121
      },
      {
        "mal_id": 40,
        "name": "Psychological",
        "url": "https://myanimelist.net/anime/genre/40/Psychological",
        "count": 427
      },
      {
        "mal_id": 3,
        "name": "Racing",
        "url": "https://myanimelist.net/anime/genre/3/Racing",
        "count": 255
      },
      {
        "mal_id": 72,
        "name": "Reincarnation",
        "url": "https://myanimelist.net/anime/genre/72/Reincarnation",
        "count": 133
      },
      {
        "mal_id": 73,
        "name": "Reverse Harem",
        "url": "https://myanimelist.net/anime/genre/73/Reverse_Harem",
        "count": 77
      },
      {
        "mal_id": 74,
        "name": "Romantic Subtext",
        "url": "https://myanimelist.net/anime/genre/74/Romantic_Subtext",
        "count": 53
      },
      {
        "mal_id": 21,
        "name": "Samurai",
        "url": "https://myanimelist.net/anime/genre/21/Samurai",
        "count": 237
      },
      {
        "mal_id": 23,
        "name": "School",
        "url": "https://myanimelist.net/anime/genre/23/School",
        "count": 2058
      },
      {
        "mal_id": 75,
        "name": "Showbiz",
        "url": "https://myanimelist.net/anime/genre/75/Showbiz",
        "count": 42
      },
      {
        "mal_id": 29,
        "name": "Space",
        "url": "https://myanimelist.net/anime/genre/29/Space",
        "count": 631
      },
      {
        "mal_id": 11,
        "name": "Strategy Game",
        "url": "https://myanimelist.net/anime/genre/11/Strategy_Game",
        "count": 328
      },
      {
        "mal_id": 31,
        "name": "Super Power",
        "url": "https://myanimelist.net/anime/genre/31/Super_Power",
        "count": 710
      },
      {
        "mal_id": 76,
        "name": "Survival",
        "url": "https://myanimelist.net/anime/genre/76/Survival",
        "count": 72
      },
      {
        "mal_id": 77,
        "name": "Team Sports",
        "url": "https://myanimelist.net/anime/genre/77/Team_Sports",
        "count": 309
      },
      {
        "mal_id": 78,
        "name": "Time Travel",
        "url": "https://myanimelist.net/anime/genre/78/Time_Travel",
        "count": 140
      },
      {
        "mal_id": 32,
        "name": "Vampire",
        "url": "https://myanimelist.net/anime/genre/32/Vampire",
        "count": 169
      },
      {
        "mal_id": 79,
        "name": "Video Game",
        "url": "https://myanimelist.net/anime/genre/79/Video_Game",
        "count": 149
      },
      {
        "mal_id": 80,
        "name": "Visual Arts",
        "url": "https://myanimelist.net/anime/genre/80/Visual_Arts",
        "count": 91
      },
      {
        "mal_id": 48,
        "name": "Workplace",
        "url": "https://myanimelist.net/anime/genre/48/Workplace",
        "count": 200
      },
      {
        "mal_id": 43,
        "name": "Josei",
        "url": "https://myanimelist.net/anime/genre/43/Josei",
        "count": 151
      },
      {
        "mal_id": 15,
        "name": "Kids",
        "url": "https://myanimelist.net/anime/genre/15/Kids",
        "count": 6483
      },
      {
        "mal_id": 42,
        "name": "Seinen",
        "url": "https://myanimelist.net/anime/genre/42/Seinen",
        "count": 1015
      },
      {
        "mal_id": 25,
        "name": "Shoujo",
        "url": "https://myanimelist.net/anime/genre/25/Shoujo",
        "count": 484
      },
      {
        "mal_id": 27,
        "name": "Shounen",
        "url": "https://myanimelist.net/anime/genre/27/Shounen",
        "count": 1948
      }]"#;

pub fn mal_genres() -> Vec<Genre> {
    let x = serde_json::from_str::<Vec<Genre>>(GENRES);
    match x {
        Err(e) => {
            log!(e.to_string());
            panic!()
        }
        Ok(v) => v,
    }
}
