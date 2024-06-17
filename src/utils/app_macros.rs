use crate::prelude::*;

// macro_rules! setup_yew {
//     () => {
//         (use_context::<AppContext>().unwrap(), use_navigator().unwrap())
//     };
// }

macro_rules! get_elem_by_id {
    ($id:expr) => {
        web_sys::window()
            .expect("Window unavailable.")
            .document()
            .expect("Document unavailable.")
            .get_element_by_id($id)
            .expect(format!("Element with id {} unavailable.", $id).as_str())
    };
}

pub(crate) use get_elem_by_id;
