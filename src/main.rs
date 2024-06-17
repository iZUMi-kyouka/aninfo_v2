use aninfo_v2::*;
// use nyaa_si::{Client, NyaaCategory, NyaaClient, QueryBuilder, Sort};

fn main() {
    yew::Renderer::<App>::new().render();
}

// #[tokio::main]
// async fn main() {
//     let query = QueryBuilder::new()
//         .search("Frieren: Beyond Journey's End 01")
//         .sort(Sort::Downloads)
//         .category(NyaaCategory::Anime)
//         .build();

//     let client = NyaaClient::new();
//     let res = client.get(&query).await.unwrap();
//     println!("{}", res.len());
//     println!("{:#?}", &res[..2]);
// }
