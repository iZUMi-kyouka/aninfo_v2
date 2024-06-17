use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TorrentPopupProps {
    pub torrents: Vec<Torrent>,
}

#[function_component]
pub fn TorrentPopup(props: &TorrentPopupProps) -> Html {
    html! {
        <div class="torrent-popup">
            {(&(props.torrents)).into_iter().map(|torrent| {
                html!{
                    <div class="torrent-card">
                        <div class="torrent-title cursor-arrow">{"Title "}{&(torrent.title)}</div>
                        <div class="torrent-size-mb cursor-arrow">{"Size "}{&(torrent.size_mb)}</div>
                        <div class="torrent-link-magnet"><a target="_blank" href={(torrent.link_magnet).clone()}>{"Magnet"}</a></div>
                        <div class="torrent-link-torrent"><a href={(torrent.link_torrent).clone()} target="_blank">{"Torrent"}</a></div>
                        <div class="torrent-download">{"Downloaded "}{&(torrent.download)}</div>
                    </div>
                }
            }).collect::<Html>()}
        </div>
    }
}
