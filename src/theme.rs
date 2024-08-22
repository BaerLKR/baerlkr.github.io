use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

pub struct Theme {
    pub base: &'static str,
    pub surface: &'static str,
    pub overlay: &'static str,
    pub muted: &'static str,
    pub subtle: &'static str,
    pub text: &'static str,
    pub love: &'static str,
    pub gold: &'static str,
    pub rose: &'static str,
    pub pine: &'static str,
    pub foam: &'static str,
    pub iris: &'static str,
    pub high_low: &'static str,
    pub high_med: &'static str,
    pub high_high: &'static str,
}
pub const ROSE_PINE: Theme = Theme {
    base: "#191724",
    surface: "#1f1d2e",
    overlay: "#26233a",
    muted: "#6e6a86",
    subtle: "#908caa",
    text: "#e0def4",
    love: "#eb6f92",
    gold: "#f6c177",
    rose: "#ebbcba",
    pine: "#31748f",
    foam: "#9ccfd8",
    iris: "#c4a7e7",
    high_low: "#21202e",
    high_med: "#403d52",
    high_high: "#524f67",
};
pub const HACKER: Theme = Theme {
    base: "#090300",
    surface: "#00ab42",
    overlay: "#006e2b",
    muted: "#7f0392",
    subtle: "#6c007d",
    text: "#7f0392",
    love: "#fff",
    gold: "#ffaf42",
    rose: "#cfff42",
    pine: "#31748f",
    foam: "#ffaf42",
    iris: "#cfff42",
    high_low: "#00ab42",
    high_med: "#403d52",
    high_high: "#05e226",
};
pub fn theme(theme: Theme) {
    let doc = window()
        .unwrap()
        .document()
        .unwrap()
        .document_element()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .style();
    doc.set_property("--base", theme.base).unwrap();
    doc.set_property("--surface", theme.surface).unwrap();
    doc.set_property("--overlay", theme.overlay).unwrap();
    doc.set_property("--muted", theme.muted).unwrap();
    doc.set_property("--subtle", theme.subtle).unwrap();
    doc.set_property("--text", theme.text).unwrap();
    doc.set_property("--love", theme.love).unwrap();
    doc.set_property("--gold", theme.gold).unwrap();
    doc.set_property("--rose", theme.rose).unwrap();
    doc.set_property("--pine", theme.pine).unwrap();
    doc.set_property("--foam", theme.foam).unwrap();
    doc.set_property("--iris", theme.iris).unwrap();
    doc.set_property("--high-low", theme.high_low).unwrap();
    doc.set_property("--high-med", theme.high_med).unwrap();
    doc.set_property("--high-high", theme.high_high).unwrap();
}
