use super::{Game, GameType, State, ViewController};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, MouseEvent};

pub(crate) struct MenuController {
    game: Game,
    view: Option<Element>,
}

impl MenuController {
    pub(crate) fn new(game: Game) -> Self {
        Self {
            game,
            view: None,
        }
    }
}

impl ViewController for MenuController {
    fn is_for_state(&self, state: State) -> bool {
        match state {
            State::Menu => true,
            _ => false,
        }
    }

    fn show(&mut self) -> Element {
        assert_eq!(self.view, None);

        let document = window().unwrap().document().unwrap();

        // create simple menu
        let view = document
            .create_element("div")
            .expect("create_element failed");
        view.set_id("menu");
        view.set_class_name("menu");

        let temperature = document
            .create_element("div")
            .expect("create_element failed");
        temperature.set_class_name("menu-button");
        temperature.set_inner_html("Temperatures");

        view.append_with_node_1(&temperature)
            .expect("append_with_node_1 failed");

        self.view = Some(view.clone());

        // attach handlers
        let click = {
            let game = self.game.clone();
            Closure::wrap(Box::new(move |_event: MouseEvent| {
                game.transition(State::Playing(GameType::Temperatures));
            }) as Box<dyn FnMut(_)>)
        };
        view
            .add_event_listener_with_callback("click", click.as_ref().unchecked_ref())
            .unwrap();
        click.forget();

        view
    }

    fn hide(&mut self) {
        if let Some(ref view) = self.view {
            view.remove();
        }
        self.view = None;
    }
}