use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlElement};

pub fn cursor_glow() {
    let js_window = window().unwrap();
    let document = js_window.document().unwrap();

    let div = document
        .get_element_by_id("follower")
        .expect("element should exist")
        .dyn_into::<HtmlElement>()
        .unwrap();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let height = div.scroll_height();
        let (x, y) = (event.client_x(), event.client_y());
        div.style()
            .set_property("left", &format!("{}px", x - height / 2))
            .unwrap();
        div.style()
            .set_property("top", &format!("{}px", y - height / 2))
            .unwrap();
    }) as Box<dyn FnMut(_)>);
    js_window
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}
