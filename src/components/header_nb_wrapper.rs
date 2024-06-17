use crate::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct WrapperProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn HNBWrapper(props: &WrapperProps) -> Html {
    let this_node = use_node_ref();
    let cx = use_context::<AppContext>().unwrap();
    let theme = handle_theme(&cx);

    let theme_cloned = theme.clone();
    use_click_away(this_node.clone(), move |_: Event| {
        // log!("clicked away");
        let left_nb = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("nb-left")
            .unwrap();
        let menu_icon = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("menu-icon")
            .unwrap();
        let cur_class = left_nb.get_attribute("class").unwrap();
        if cur_class.contains("nb-left-active") {
            let menu_icon = menu_icon.clone();
            left_nb
                .set_attribute("class", &format!("nb-left {}", theme_cloned.clone()))
                .unwrap();
            menu_icon
                .set_attribute("class", &format!("icon-{} rotate-half", theme_cloned))
                .unwrap();
            gloo::timers::callback::Timeout::new(75, move || {
                menu_icon.set_attribute("src", "./static/menu.svg").unwrap();
            })
            .forget();
        }
    });

    html! {
        <div class="display-flex" ref={this_node}>
        {props.children.clone()}
        </div>
    }
}
