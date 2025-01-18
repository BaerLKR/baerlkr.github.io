use compile_time::unix;
use wasm_timer::{SystemTime, UNIX_EPOCH};
use web_sys::window;

pub fn render() {
    let comp_t = unix!().to_string();
    let win = window().unwrap();
    let doc = win.document().unwrap();
    doc.get_element_by_id("comp_time")
        .unwrap()
        .set_inner_html(&comp_t);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    doc.get_element_by_id("render_time")
        .unwrap()
        .set_inner_html(&now.as_secs().to_string());
}
