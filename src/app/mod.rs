use crate::logic::{GameSummary, Quantity};

use super::log;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use cards::CardsController;
use menu::MenuController;
use wasm_bindgen::JsCast;
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure};
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
    pub fn add_event_listener<VC, F, E>(&mut self, element: &Element, event_name: &str, mut callback: F)
    where
        F: 'static + FnMut(&mut VC, E) -> Option<Reaction>,
        E: 'static + Clone + FromWasmAbi,
        AppController: VCMapper<VC>,
    {
        let closure = {
            let controller = self.controller.clone();
            Closure::wrap(Box::new(move |event: E| {
                let reaction = if let Some(app_controller) = controller.upgrade() {
                    app_controller
                        .borrow_mut()
                        .map_vc(|vc: &mut VC| callback(vc, event.clone()))
                } else {
                    None
                };
                if let Some(Some(reaction)) = reaction {
                    if let Some(app_controller) = controller.upgrade() {
                        AppController::react(app_controller, reaction);
                    }
                }
            }) as Box<dyn FnMut(_)>)
        };
        element
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    pub fn add_event_reaction(&mut self, element: &Element, event_name: &str, reaction: Reaction) {
        let closure = {
            let controller = self.controller.clone();
            Closure::wrap(Box::new(move || {
                if let Some(app_controller) = controller.upgrade() {
                    AppController::react(app_controller, reaction);
                }
            }) as Box<dyn FnMut()>)
        };
        element
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}

impl From<Weak<RefCell<AppController>>> for Presenter {
    fn from(controller: Weak<RefCell<AppController>>) -> Self {
        Self { controller }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Reaction {
    Transition(State),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Menu,
    Settings,
    Playing(Quantity),
    Ended(GameSummary),
}

trait ViewController {
    fn show(&mut self, presenter: Presenter) -> Element;
    fn hide(&mut self);
}

pub struct AppController {
    content: Element,
    menu_controller: Option<MenuController>,
    cards_controller: Option<CardsController>,
}

pub trait VCMapper<VC> {
    fn map_vc<F, R>(&mut self, mapper: F) -> Option<R>
    where
        F: FnMut(&mut VC) -> R;
    fn set_vc(&mut self, vc: VC);
}

impl VCMapper<MenuController> for AppController {
    fn map_vc<F, R>(&mut self, mut mapper: F) -> Option<R>
    where
        F: FnMut(&mut MenuController) -> R,
    {
        if let Some(ref mut ctrl) = self.menu_controller {
            Some(mapper(ctrl))
        } else {
            None
        }
    }

    fn set_vc(&mut self, vc: MenuController) {
        assert!(self.menu_controller.is_none());
        self.menu_controller = Some(vc);
    }
}

impl VCMapper<CardsController> for AppController {
    fn map_vc<F, R>(&mut self, mut mapper: F) -> Option<R>
    where
        F: FnMut(&mut CardsController) -> R,
    {
        if let Some(ref mut ctrl) = self.cards_controller {
            Some(mapper(ctrl))
        } else {
            None
        }
    }

    fn set_vc(&mut self, vc: CardsController) {
        assert!(self.cards_controller.is_none());
        self.cards_controller = Some(vc);
    }
}

impl AppController {
    fn new(content: Element) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            content,
            menu_controller: None,
            cards_controller: None,
        }))
    }

    fn react(self_: Rc<RefCell<Self>>, reaction: Reaction) {
        match reaction {
            Reaction::Transition(state) => AppController::transition(self_, state),
        }
    }

    fn transition(self_: Rc<RefCell<Self>>, state: State) {
        log!("Transitioning to: {:?}", state);
        match state {
            State::Menu => AppController::show_view_controller(self_, MenuController::default()),
            State::Settings => {}
            State::Playing(game_type) => AppController::show_view_controller(self_, CardsController::new(game_type)),
            State::Ended(_) => {}
        }
    }

    fn show_view_controller<VC>(self_: Rc<RefCell<Self>>, mut view_controller: VC)
    where
        AppController: VCMapper<VC>,
        VC: ViewController,
    {
        let presenter = Rc::downgrade(&self_).into();
        let mut controller = self_.borrow_mut();
        if let Some(ref mut sub_controller) = controller.menu_controller {
            sub_controller.hide();
        }
        controller.menu_controller = None;
        if let Some(ref mut sub_controller) = controller.cards_controller {
            sub_controller.hide();
        }
        controller.cards_controller = None;
        controller
            .content
            .append_with_node_1(&view_controller.show(presenter))
            .expect("append_with_node_1 failed");
        controller.set_vc(view_controller);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
