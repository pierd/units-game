use std::cell::RefCell;
use std::rc::Rc;

use game::Game;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Element, MouseEvent, TouchEvent};

mod game;
mod logging;

trait PointerEvent {
    fn get_x(&self) -> i32;
}

impl PointerEvent for MouseEvent {
    fn get_x(&self) -> i32 {
        self.client_x()
    }
}

impl PointerEvent for TouchEvent {
    fn get_x(&self) -> i32 {
        self.touches().item(0).unwrap().client_x()
    }
}

#[wasm_bindgen]
pub struct Module {
    game: Rc<RefCell<Game>>,
    content: Element,
}

#[wasm_bindgen]
impl Module {
    #[wasm_bindgen(constructor)]
    pub fn new(content_id: String) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let element = document
            .get_element_by_id(&content_id)
            .expect("get_element_by_id failed");
        Self {
            game: Game::new(element.clone()),
            content: element,
        }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log!("Starting in: {}", self.content.id());
        game::Game::run(self.game.clone());
        log!("Started.");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
