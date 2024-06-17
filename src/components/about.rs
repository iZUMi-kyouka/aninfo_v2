use crate::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);
    use_title("About | ANiNFO".to_string());

    html! {
        <div class={format!("about-wrapper written-content {}", theme)}>
            {match (*cx).get_langauge() {
                Language::EN => html!{
                    <>
                    <h2 class="content-header">{"About ANiNFO"}</h2>
                    <p>{"ANiNFO is an anime website offering users the service of retrieving information about anime."}</p>
                    <h3>{"Why do we exist?"}</h3>
                    <p>{"We strive to offer a one-stop service where users can get information about a specific anime that is hassle-free.
                    This project also serves as a prove that Rust is production-ready!"}</p>
                    <h3>{"Technical Information"}</h3>
                    <p>{"This service is a mostly frontend-only service written in Rust using the Yew.rs framework. The source code are then compiled to WASM using Trunk.
                    The service is offered by Jikan REST"}</p>
                    <h3>{"Attribution"}</h3>
                    {"This application retrieves MyAnimeList's data using the Jikan REST API.
                    This application uses icons from Flaticon to build the user interface, with sources from each individual icon to be updated."}
                    </>
                },
                Language::JP => html!{
                    <>
                    <h2 class="content-header">{"ANiNFOについて"}</h2>
                    {"ANiNFOは、"}<ruby>{"アニメ"}<rp> </rp><rt>{"あにめ"}</rt><rp> </rp></ruby><ruby>{"に関する"}<rp> </rp><rt>{"にかんする"}</rt><rp> </rp></ruby><ruby>{"情報"}<rp> </rp><rt>{"じょうほう"}</rt><rp> </rp></ruby>{"を"}<ruby>{"検索"}<rp> </rp><rt>{"けんさく"}</rt><rp> </rp></ruby>{"する"}<ruby>{"サービス"}<rp> </rp><rt>{"さあびす"}</rt><rp> </rp></ruby>{"を"}<ruby>{"提供"}<rp> </rp><rt>{"ていきょう"}</rt><rp> </rp></ruby>{"する"}<ruby>{"アニメ"}<rp> </rp><rt>{"あにめ"}</rt><rp> </rp></ruby><ruby>{"サイト"}<rp> </rp><rt>{"さいと"}</rt><rp> </rp></ruby>{"です。"}<br />

                    <h3>{"なぜ"}<ruby>{"私"}<rp> </rp><rt>{"わたし"}</rt><rp> </rp></ruby>{"たちが"}<ruby>{"存在"}<rp> </rp><rt>{"そんざい"}</rt><rp> </rp></ruby>{"するのか？"}</h3>
                    {"私たちは、"}<ruby>{"ユーザー"}<rp> </rp><rt>{"ゆうざあ"}</rt><rp> </rp></ruby>{"が"}<ruby>{"特定"}<rp> </rp><rt>{"とくてい"}</rt><rp> </rp></ruby>{"の"}<ruby>{"アニメ"}<rp> </rp><rt>{"あにめ"}</rt><rp> </rp></ruby>{"の"}<ruby>{"情報"}<rp> </rp><rt>{"じょうほう"}</rt><rp> </rp></ruby>{"を"}<ruby>{"手間"}<rp> </rp><rt>{"てま"}</rt><rp> </rp></ruby>{"なく"}<ruby>{"ワン"}<rp> </rp><rt>{"わん"}</rt><rp> </rp></ruby><ruby>{"ストップ"}<rp> </rp><rt>{"すとっぷ"}</rt><rp> </rp></ruby>{"で"}<ruby>{"得"}<rp> </rp><rt>{"え"}</rt><rp> </rp></ruby>{"られる"}<ruby>{"サービス"}<rp> </rp><rt>{"さあびす"}</rt><rp> </rp></ruby>{"を"}<ruby>{"提供"}<rp> </rp><rt>{"ていきょう"}</rt><rp> </rp></ruby>{"することを"}<ruby>{"目指し"}<rp> </rp><rt>{"めざし"}</rt><rp> </rp></ruby>{"ます。また、"}<ruby>{"ラスト"}<rp> </rp><rt>{"らすと"}</rt><rp> </rp></ruby>{"が"}<ruby>{"制作"}<rp> </rp><rt>{"せいさく"}</rt><rp> </rp></ruby><ruby>{"可能"}<rp> </rp><rt>{"かのう"}</rt><rp> </rp></ruby>{"であることを"}<ruby>{"証明"}<rp> </rp><rt>{"しょうめい"}</rt><rp> </rp></ruby>{"する"}<ruby>{"プロジェクト"}<rp> </rp><rt>{"ぷろじぇくと"}</rt><rp> </rp></ruby>{"でもあります！"}<br />
                    <h3><ruby>{"技術"}<rp> </rp><rt>{"ぎじゅつ"}</rt><rp> </rp></ruby><ruby>{"情報"}<rp> </rp><rt>{"じょうほう"}</rt><rp> </rp></ruby></h3>
                    {"この"}<ruby>{"サービス"}<rp> </rp><rt>{"さあびす"}</rt><rp> </rp></ruby>{"は、Yew.rs"}<ruby>{"フレーム"}<rp> </rp><rt>{"ふれえむ"}</rt><rp> </rp></ruby><ruby>{"ワーク"}<rp> </rp><rt>{"わあく"}</rt><rp> </rp></ruby>{"を"}<ruby>{"使っ"}<rp> </rp><rt>{"つかっ"}</rt><rp> </rp></ruby>{"て"}<ruby>{"Rust"}<rp> </rp><rt>{"るすと"}</rt><rp> </rp></ruby>{"で"}<ruby>{"書か"}<rp> </rp><rt>{"かか"}</rt><rp> </rp></ruby>{"れた、ほとんど"}<ruby>{"フロント"}<rp> </rp><rt>{"ふろんと"}</rt><rp> </rp></ruby><ruby>{"エンド"}<rp> </rp><rt>{"えんど"}</rt><rp> </rp></ruby>{"のみの"}<ruby>{"サービス"}<rp> </rp><rt>{"さあびす"}</rt><rp> </rp></ruby>{"です。"}<ruby>{"ソース"}<rp> </rp><rt>{"そーす"}</rt><rp> </rp></ruby><ruby>{"コード"}<rp> </rp><rt>{"こーど"}</rt><rp> </rp></ruby>{"は"}<ruby>{"Trunk"}<rp> </rp><rt>{"とらんく"}</rt><rp> </rp></ruby>{"を"}<ruby>{"使っ"}<rp> </rp><rt>{"つかっ"}</rt><rp> </rp></ruby>{"て"}<ruby>{"WASM"}<rp> </rp><rt>{"わずむ"}</rt><rp> </rp></ruby>{"に"}<ruby>{"コンパイル"}<rp> </rp><rt>{"こんぱいる"}</rt><rp> </rp></ruby>{"されます。この"}<ruby>{"サービス"}<rp> </rp><rt>{"さあびす"}</rt><rp> </rp></ruby>{"はJikan RESTによって"}<ruby>{"提供"}<rp> </rp><rt>{"ていきょう"}</rt><rp> </rp></ruby>{"されます。"}
                    </>
                },
            }}

        </div>
    }
}
