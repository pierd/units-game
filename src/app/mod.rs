use super::log;
use crate::logic::GameType;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use cards::CardsController;
use menu::MenuController;
use web_sys::Element;

mod cards;
mod gestures;
mod menu;

#[derive(Clone)]
pub struct App {
    controller: Rc<RefCell<AppController>>,
}

impl App {
    pub fn new(content: Element) -> Self {
        Self::wrap(AppController::new(content))
    }

    fn wrap(controller: Rc<RefCell<AppController>>) -> Self {
        Self { controller }
    }

    pub fn run(&self) {
        self.transition(State::Menu);
    }

    pub fn transition(&self, state: State) {
        AppController::transition(self.controller.clone(), state);
    }
}

#[derive(Clone)]
pub struct Presenter {
    controller: Weak<RefCell<AppController>>,
}

impl Presenter {
    pub fn transition(&self, state: State) {
        self.controller.upgrade().map(|controller| {
            AppController::transition(controller, state);
        });
    }
}

impl From<Weak<RefCell<AppController>>> for Presenter {
    fn from(controller: Weak<RefCell<AppController>>) -> Self {
        Self { controller }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Menu,
    Playing(GameType),
}

trait ViewController {
    fn show(&mut self, presenter: Presenter) -> Element;
    fn hide(&mut self);
}

struct AppController {
    content: Element,
    sub_controller: Option<Box<dyn ViewController>>,
}

impl AppController {
    fn new(content: Element) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            content,
            sub_controller: None,
        }))
    }

    fn transition(self_: Rc<RefCell<Self>>, state: State) {
        log!("Transitioning to: {:?}", state);
        AppController::show_view_controller(self_, AppController::create_view_controller(state));
    }

    fn create_view_controller(state: State) -> Box<dyn ViewController> {
        match state {
            State::Menu => Box::new(MenuController::default()),
            State::Playing(game_type) => Box::new(CardsController::new(game_type)),
        }
    }

    fn show_view_controller(self_: Rc<RefCell<Self>>, mut view_controller: Box<dyn ViewController>) {
        let presenter = Rc::downgrade(&self_).into();
        let mut controller = self_.borrow_mut();
        if let Some(ref mut sub_controller) = controller.sub_controller {
            sub_controller.hide();
        }
        controller
            .content
            .append_with_node_1(&view_controller.show(presenter))
            .expect("append_with_node_1 failed");
        controller.sub_controller = Some(view_controller);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
