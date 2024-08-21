use crate::random;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlElement};

const LENGTH: usize = ALPHABET.len();
const ALPHABET: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890#$%&/()=?!+*~{}[]";
const NAME: &str = "Lovis Rentsch";

pub fn text_animation() {
    const ANIMATION_INTERVAL: i32 = 120;
    let animation_ongoing = Rc::new(Cell::new(false));
    let js_window = window().unwrap();
    let document = js_window.document().unwrap();
    let name_element = document.get_element_by_id("name").unwrap();
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
    let end_animation = Closure::wrap({
        let animation_ongoing = animation_ongoing.clone();
        Box::new(move || {
            animation_ongoing.replace(false);
        }) as Box<dyn FnMut()>
    });
    let name_effect = Closure::wrap({
        let animation_ongoing = animation_ongoing.clone();
        Box::new(move |_event: web_sys::MouseEvent| {
            if *animation_ongoing == false.into() {
                animation_ongoing.replace(true);
                let window = window().unwrap();
                let elem = document
                    .get_element_by_id("name")
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap();
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        end_animation.as_ref().unchecked_ref(),
                        (NAME.len() as i32 * 2) * ANIMATION_INTERVAL,
                    )
                    .unwrap();
                for i in 0..(NAME.len() as i32 * 2) {
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_2(
                            anim.as_ref().unchecked_ref(),
                            i * ANIMATION_INTERVAL,
                            &i.into(),
                            &elem.to_owned(),
                        )
                        .unwrap();
                }
            }
        }) as Box<dyn FnMut(_)>
    });

    name_element
        .add_event_listener_with_callback("mouseover", name_effect.as_ref().unchecked_ref())
        .unwrap();
    name_effect.forget();
}
