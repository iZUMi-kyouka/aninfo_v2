use crate::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TorrentProps {
    torrent_response: TorrentResponse,
}

#[function_component]
pub fn Torrents(props: &TorrentProps) -> Html {
    html! {
        {serde_json::to_string_pretty(&(props.torrent_response)).unwrap_or("Error parsing TorrentResponse object.".to_string())}
    }
}

#[function_component]
pub fn TorrentEmpty() -> Html {
    html!()
}
