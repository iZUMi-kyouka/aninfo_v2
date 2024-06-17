use crate::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CharacterCardProp {
    pub char: Char,
}

#[function_component(CharacterCard)]
pub fn char_card(props: &CharacterCardProp) -> Html {
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);

    html! {
        <div class={format!("char-card hover-highlight {}", theme)}>
            <a target="_blank" href={props.char.character.url.clone()}><img class="char-card-img" width="170.35px" height="265px" loading="lazy"
            src={props.char.character.get_image_webp().unwrap_or(props.char.character.get_images_jpg().unwrap())} class="char-img"/></a>
            <a target="_blank" href={props.char.character.url.clone()}><h5 class="char-title cursor-pointer">{handle_long_name(props.char.character.get_name().as_str())}</h5></a>
            <p class="char-role">{props.char.role.clone()}</p>
        </div>
    }
}
