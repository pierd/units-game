use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Element};

mod game;
mod logging;
mod logic;

#[wasm_bindgen]
pub struct Module {
    game: game::Game,
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
            game: game::Game::new(element.clone()),
            content: element,
        }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log!("Starting in: {}", self.content.id());
        self.game.run();
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
