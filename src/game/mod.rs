use super::log;

use std::cell::RefCell;
use std::rc::Rc;

use cards::CardsController;
use menu::MenuController;
use web_sys::Element;

mod cards;
mod gestures;
mod menu;

#[derive(Clone)]
pub(crate) struct Game {
    controller: Rc<RefCell<GameController>>,
}

impl Game {
    pub fn new(content: Element) -> Self {
        Self::wrap(GameController::new(content))
    }

    fn wrap(controller: Rc<RefCell<GameController>>) -> Self {
        Self { controller }
    }

    pub fn run(&self) {
        self.transition(State::Menu);
    }

    pub(crate) fn transition(&self, state: State) {
        GameController::transition(self.controller.clone(), state);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GameType {
    Temperatures
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum State {
    Menu,
    Playing(GameType),
}

trait ViewController {
    fn is_for_state(&self, state: State) -> bool;
    fn show(&mut self) -> Element;
    fn hide(&mut self);
}

struct GameController {
    content: Element,
    sub_controller: Option<Box<dyn ViewController>>,
}

impl GameController {
    fn new(content: Element) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            content,
            sub_controller: None,
        }))
    }

    fn transition(self_: Rc<RefCell<Self>>, state: State) {
        log!("Transitioning to: {:?}", state);
        self_.borrow_mut().show_view_controller(GameController::create_view_controller(self_.clone(), state));
    }

    fn create_view_controller(self_: Rc<RefCell<Self>>, state: State) -> Box<dyn ViewController> {
        match state {
            State::Menu => Box::new(MenuController::new(Game::wrap(self_.clone()))),
            State::Playing(game_type) => Box::new(CardsController::new(Game::wrap(self_.clone()), game_type)),
        }
    }

    fn show_view_controller(&mut self, mut view_controller: Box<dyn ViewController>) {
        if let Some(ref mut sub_controller) = self.sub_controller {
            sub_controller.hide();
        }
        self.content.append_with_node_1(&view_controller.show())
            .expect("append_with_node_1 failed");
        self.sub_controller = Some(view_controller);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
