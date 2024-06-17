use crate::prelude::*;

#[function_component]
pub fn ContextDebugger() -> Html {
    let cx = use_context::<AppContext>().unwrap();
    html! {
        {to_string_pretty(&(*cx).clone()).unwrap()}
    }
}
