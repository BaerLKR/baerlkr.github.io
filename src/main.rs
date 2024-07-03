#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlElement;

const LENGTH: usize = ALPHABET.len();
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const NAME: &str = "Lovis Rentsch";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

fn start_app() -> Result<(), JsValue> {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");

    let div = document
        .get_element_by_id("follower")
        .expect("element should exist")
        .dyn_into::<HtmlElement>()
        .unwrap();
    let name_element = document.get_element_by_id("name").unwrap();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let height = div.scroll_height();
        let x = event.client_x();
        let y = event.client_y();
        div.style()
            .set_property("left", &format!("{}px", x - height / 2))
            .unwrap();
        div.style()
            .set_property("top", &format!("{}px", y - height / 2))
            .unwrap();
    }) as Box<dyn FnMut(_)>);
    let anim = Closure::wrap(Box::new(move |iteration: usize, element: HtmlElement| {
        element.set_text_content(Some({
            &NAME
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if iteration >= NAME.len() + i {
                        c
                    } else if i < iteration {
                        ALPHABET
                            .chars()
                            .nth((random() * LENGTH as f32) as usize)
                            .unwrap()
                    } else {
                        c
                    }
                })
                .collect::<String>()
        }));
    }) as Box<dyn FnMut(_, _)>);
    let name_effect = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let window = window().unwrap();
        let elem = document
            .get_element_by_id("name")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        for i in 0..(NAME.len() as i32 * 2) {
            window
                .set_timeout_with_callback_and_timeout_and_arguments_2(
                    anim.as_ref().unchecked_ref(),
                    i * 100,
                    &i.into(),
                    &elem.to_owned(),
                )
                .unwrap();
        }
    }) as Box<dyn FnMut(_)>);
    let window = window().unwrap();
    window.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    name_element
        .add_event_listener_with_callback("mouseover", name_effect.as_ref().unchecked_ref())?;
    name_effect.forget();
    closure.forget();
    Ok(())
}
fn anim(element: &mut HtmlElement, iteration: &usize) {
    element.set_text_content(Some({
        &NAME
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i < *iteration {
                    ALPHABET
                        .chars()
                        .nth((random() * LENGTH as f32) as usize)
                        .unwrap()
                } else {
                    c
                }
            })
            .collect::<String>()
    }))
}

fn main() {
    set_panic_hook();
    start_app().unwrap();
}
