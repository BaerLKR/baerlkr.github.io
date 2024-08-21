use crate::{
    random,
    theme::{theme, HACKER, ROSE_PINE},
    ALPHABET, LENGTH,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, CanvasRenderingContext2d};

pub fn matrix(cleanup: Option<i32>) -> Option<i32> {
    let win = window().unwrap();
    let doc = win.document().unwrap();
    let canvas = doc
        .get_element_by_id("matrix-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    if let Some(handle) = cleanup {
        win.clear_interval_with_handle(handle);
        theme(ROSE_PINE);
        canvas.style().set_property("display", "none").unwrap();
        None
    } else {
        theme(HACKER);
        canvas.style().set_property("display", "block").unwrap();
        canvas.set_height(win.inner_height().unwrap().as_f64().unwrap() as u32);
        canvas.set_width(win.inner_width().unwrap().as_f64().unwrap() as u32);
        let ctx = Rc::new(RefCell::new(
            canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap(),
        ));
        let (height, width) = (canvas.scroll_height(), canvas.scroll_width());
        let font_size = 16;
        let columns = width / font_size;
        let drops = Rc::new(RefCell::new(vec![
            (random() * height as f32) as i32;
            columns as usize
        ]));
        let callback = Closure::wrap(Box::new(move || {
            draw(ctx.clone(), drops.clone(), &width, &height, &font_size);
        }) as Box<dyn Fn()>);
        let handle = window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                33,
            )
            .unwrap();
        callback.forget();
        Some(handle)
    }
}
fn draw(
    ctx: Rc<RefCell<CanvasRenderingContext2d>>,
    drops: Rc<RefCell<Vec<i32>>>,
    width: &i32,
    height: &i32,
    font_size: &i32,
) {
    let ctx: std::cell::Ref<CanvasRenderingContext2d> = (*ctx).borrow();
    let mut drops = (*drops).borrow_mut();
    ctx.set_font("15pt monospace");
    ctx.set_fill_style(&"rgba(0,0,0,.1)".into());
    ctx.fill_rect(0., 0., (*width).into(), (*height).into());
    for i in 0..drops.len() {
        let text = ALPHABET
            .chars()
            .nth((random() * LENGTH as f32) as usize)
            .unwrap();
        ctx.set_fill_style(&HACKER.high_high.into());
        ctx.fill_text(
            text.to_string().as_str(),
            (i * *font_size as usize) as f64,
            (drops[i] * font_size) as f64,
        )
        .unwrap();
        drops[i] += 1;
        if drops[i] * font_size > *height && random() > 0.95 {
            drops[i] = 0;
        }
    }
}
pub fn rust() {
    let mut nix = false;
    let window = window().unwrap();
    let rust_elem = window
        .document()
        .unwrap()
        .get_element_by_id("rust")
        .unwrap();
    let mut handle = 0;
    let rust_effect = Closure::wrap(Box::new(move || {
        nix = !nix;
        let r = window
            .document()
            .unwrap()
            .get_element_by_id("rust")
            .unwrap();
        if nix {
            r.set_inner_html(NIX);
            handle = matrix(None).unwrap();
        } else {
            r.set_inner_html(RUST);
            matrix(Some(handle));
        }
    }) as Box<dyn FnMut()>);
    rust_elem
        .add_event_listener_with_callback("click", rust_effect.as_ref().unchecked_ref())
        .unwrap();
    rust_effect.forget();
}
// both licensed under CC0
const NIX: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><path fill="currentColor" d="m7.352 1.592l-1.364.002L5.32 2.75l1.557 2.713l-3.137-.008l-1.32 2.34h11.69l-1.353-2.332l-3.192-.006l-2.214-3.865zm6.175 0l-2.687.025l5.846 10.127l1.341-2.34l-1.59-2.765l2.24-3.85l-.683-1.182h-1.336l-1.57 2.705l-1.56-2.72zm6.887 4.195l-5.846 10.125l2.696-.008l1.601-2.76l4.453.016l.682-1.183l-.666-1.157l-3.13-.008L21.778 8.1l-1.365-2.313zM9.432 8.086l-2.696.008l-1.601 2.76l-4.453-.016L0 12.02l.666 1.157l3.13.008l-1.575 2.71l1.365 2.315zM7.33 12.25l-.006.01l-.002-.004l-1.342 2.34l1.59 2.765l-2.24 3.85l.684 1.182H7.35l.004-.006h.001l1.567-2.698l1.558 2.72l2.688-.026l-.004-.006h.01zm2.55 3.93l1.354 2.332l3.192.006l2.215 3.865l1.363-.002l.668-1.156l-1.557-2.713l3.137.008l1.32-2.34z"/></svg>"#;
const RUST: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32"><path fill="currentColor" d="m31.584 15.615l-1.329-.823a6.54 6.54 0 0 0-.036-.385l1.141-1.063a.463.463 0 0 0-.152-.765l-1.459-.543a8.832 8.832 0 0 0-.113-.38l.905-1.26a.454.454 0 0 0-.296-.719l-1.537-.251c-.057-.115-.12-.229-.181-.344l.645-1.416a.45.45 0 0 0-.036-.443a.464.464 0 0 0-.396-.203l-1.563.057a6.608 6.608 0 0 0-.245-.303l.36-1.516a.46.46 0 0 0-.552-.552l-1.516.36a5.584 5.584 0 0 0-.303-.245l.057-1.563a.455.455 0 0 0-.645-.432l-1.416.645c-.115-.061-.229-.124-.349-.181l-.251-1.537a.458.458 0 0 0-.719-.296l-1.26.905c-.125-.041-.251-.077-.375-.113L19.415.796a.458.458 0 0 0-.76-.157l-1.063 1.147a7.186 7.186 0 0 0-.391-.041L16.384.421a.454.454 0 0 0-.776 0l-.823 1.324c-.131.009-.26.025-.385.041L13.337.639a.46.46 0 0 0-.765.157l-.543 1.453c-.129.036-.249.077-.38.113l-1.26-.905a.458.458 0 0 0-.719.296L9.419 3.29c-.115.057-.228.12-.343.181l-1.417-.645a.455.455 0 0 0-.645.432l.052 1.563c-.099.079-.199.161-.297.245l-1.52-.36a.458.458 0 0 0-.548.552l.355 1.516c-.084.099-.161.197-.245.303L3.254 7.02a.456.456 0 0 0-.432.645l.647 1.416l-.188.344l-1.536.251a.46.46 0 0 0-.297.719l.912 1.26a8.585 8.585 0 0 0-.115.38l-1.459.543a.46.46 0 0 0-.151.765l1.14 1.063a6.621 6.621 0 0 0-.036.385l-1.328.823a.449.449 0 0 0 0 .771l1.328.823c.009.131.02.261.036.385l-1.14 1.068a.454.454 0 0 0 .151.76l1.459.548c.036.124.072.249.115.375l-.912 1.26a.458.458 0 0 0 .301.719l1.537.251c.057.115.12.229.183.344l-.647 1.416a.456.456 0 0 0 .432.645l1.557-.052c.084.1.161.199.251.297l-.36 1.521a.456.456 0 0 0 .548.547l1.52-.355c.099.084.199.161.303.245l-.057 1.557a.453.453 0 0 0 .645.432l1.417-.645c.115.061.228.124.343.187l.256 1.537a.455.455 0 0 0 .713.296l1.265-.911c.125.041.251.077.375.113l.548 1.459a.452.452 0 0 0 .76.152l1.063-1.141c.129.016.255.032.385.041l.823 1.328a.457.457 0 0 0 .776 0l.823-1.328c.131-.009.256-.025.385-.041l1.063 1.141a.456.456 0 0 0 .76-.152l.548-1.459c.124-.036.249-.072.375-.113l1.26.911a.456.456 0 0 0 .719-.296l.251-1.537c.12-.063.235-.125.349-.187l1.416.645a.457.457 0 0 0 .645-.432l-.057-1.557c.105-.084.204-.161.297-.245l1.521.355a.46.46 0 0 0 .552-.547l-.36-1.521c.084-.099.167-.197.245-.297l1.563.052a.455.455 0 0 0 .432-.645l-.645-1.416c.061-.115.124-.229.181-.344l1.537-.251a.458.458 0 0 0 .296-.719l-.905-1.26l.113-.375l1.453-.548a.455.455 0 0 0 .157-.76l-1.141-1.068c.011-.124.027-.255.036-.385l1.329-.823a.45.45 0 0 0 0-.771zm-8.881 11c-1.224-.26-.833-2.099.396-1.839a.94.94 0 0 1-.396 1.839m-.448-3.047a.853.853 0 0 0-1.015.661l-.475 2.192A11.485 11.485 0 0 1 16 27.453a11.52 11.52 0 0 1-4.864-1.073l-.475-2.197a.848.848 0 0 0-1.016-.656l-1.937.411a10.809 10.809 0 0 1-1.005-1.183h9.443c.105 0 .177-.015.177-.115v-3.337c0-.1-.072-.115-.177-.115h-2.765v-2.12h2.989c.271 0 1.459.077 1.833 1.593c.12.464.381 1.979.557 2.464c.177.547.901 1.629 1.672 1.629h4.704a.98.98 0 0 0 .171-.015a11.09 11.09 0 0 1-1.067 1.255zM9.197 26.573a.95.95 0 0 1-1.119-.724a.933.933 0 0 1 .724-1.115a.94.94 0 0 1 .395 1.839M5.615 12.047a.945.945 0 0 1-.464 1.271a.948.948 0 0 1-1.255-.505c-.459-1.124 1.192-1.859 1.719-.765zm-1.099 2.609l2.02-.896c.433-.192.625-.697.433-1.129l-.417-.943h1.641v7.38H4.89a11.572 11.572 0 0 1-.375-4.412zm8.869-.713v-2.177h3.896c.203 0 1.421.235 1.421 1.147c0 .76-.937 1.031-1.703 1.031zm14.167 1.958c0 .287-.011.572-.031.853h-1.188c-.12 0-.167.079-.167.199v.541c0 1.281-.719 1.557-1.355 1.631c-.604.068-1.271-.251-1.353-.62c-.355-2-.948-2.427-1.885-3.167c1.161-.74 2.369-1.823 2.369-3.281c0-1.573-1.079-2.563-1.812-3.047c-1.032-.683-2.172-.817-2.48-.817H7.395a11.55 11.55 0 0 1 6.459-3.652l1.448 1.521a.854.854 0 0 0 1.208.025l1.615-1.547a11.55 11.55 0 0 1 7.911 5.631l-1.109 2.5a.863.863 0 0 0 .437 1.131l2.131.947c.036.381.052.761.052 1.152zM15.303 3.255c.905-.864 2.203.495 1.296 1.36c-.901.781-2.115-.495-1.296-1.36m10.984 8.838a.937.937 0 0 1 1.235-.479c.771.339.744 1.437-.037 1.74c-.787.301-1.541-.495-1.197-1.261z"/>
        </svg>"#;
