use super::gestures::PointerEvent;
use super::{log, Presenter, State, ViewController};
use crate::logic::{Challenge, ChoiceSelection, Game, GameType};

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, MouseEvent, TouchEvent};

pub struct CardsController {
    controller: Rc<RefCell<CardsControllerImpl>>,
}

impl CardsController {
    pub fn new(game_type: GameType) -> Self {
        Self {
            controller: CardsControllerImpl::new(game_type),
        }
    }
}

impl ViewController for CardsController {
    fn show(&mut self, presenter: Presenter) -> Element {
        CardsControllerImpl::show(self.controller.clone(), presenter)
    }

    fn hide(&mut self) {
        log!("hiding cards pre borrow");
        self.controller.borrow_mut().hide();
    }
}

struct Card {
    card: Element,
    left: Element,
    right: Element,
}

impl Card {
    fn new(challenge: Challenge) -> Self {
        let document = window().unwrap().document().unwrap();

        // create card view
        let card = document.create_element("div").expect("create_element failed");
        card.set_class_name("card");

        // create left side of the card
        let left_choice = challenge.left_choice;
        let left = document.create_element("div").expect("create_element failed");
        left.set_class_name("left");
        left.set_inner_html(&format!("{} {}", left_choice.value, left_choice.unit));
        card.append_with_node_1(&left).expect("append_with_node_1 failed");

        // create right side of the card
        let right_choice = challenge.right_choice;
        let right = document.create_element("div").expect("create_element failed");
        right.set_class_name("right");
        right.set_inner_html(&format!("{} {}", right_choice.value, right_choice.unit));
        card.append_with_node_1(&right).expect("append_with_node_1 failed");

        Self { card, left, right }
    }

    fn set_translate(&mut self, translate_x: i32) {
        self.card
            .set_attribute(
                "style",
                &format!(
                    "transform: translate({}px, 0px) rotate({}deg);",
                    translate_x,
                    translate_x as f32 / 10.0
                ),
            )
            .expect("set style failed");

        let scale_adjust = translate_x as f32 / 100.0;
        let left_scale = if scale_adjust > 1.0 { 0.0 } else { 1.0 - scale_adjust };
        self.left
            .set_attribute("style", &format!("transform: scale({}, {});", left_scale, left_scale))
            .expect("set style failed");
        let right_scale = if scale_adjust < -1.0 { 0.0 } else { 1.0 + scale_adjust };
        self.right
            .set_attribute("style", &format!("transform: scale({}, {});", right_scale, right_scale))
            .expect("set style failed");
    }
}

#[derive(Clone, Copy, Debug)]
enum PostGestureAction {
    TransitionApp(State),
}

impl PostGestureAction {
    fn perform(&self, presenter: &Presenter) {
        log!("performing action: {:?}", self);
        match self {
            PostGestureAction::TransitionApp(state) => {
                // cloning the app to break the stack of borrowing the controller
                presenter.transition(*state)
            }
        }
    }
}

struct CardsControllerImpl {
    game: Game,
    view: Option<Element>,

    pan_start_x: Option<i32>,
    translate_x: i32,
    card: Option<Card>,
}

impl CardsControllerImpl {
    fn new(game_type: GameType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            game: Game::new(game_type),
            view: None,
            pan_start_x: None,
            translate_x: 0,
            card: None,
        }))
    }

    fn show(self_: Rc<RefCell<Self>>, presenter: Presenter) -> Element {
        let mut controller = self_.borrow_mut();
        assert_eq!(controller.view, None);

        let document = window().unwrap().document().unwrap();

        // create main container for cards
        let view = document.create_element("div").expect("create_element failed");
        view.set_class_name("cards");
        controller.view = Some(view.clone());

        // create card and add it to the view
        let card = Card::new(controller.game.challenge);
        controller.replace_card(card);

        // release the controller borrow
        let _ = controller;

        // attach gestures
        let mouse_move = {
            let controller = Rc::downgrade(&self_);
            Closure::wrap(Box::new(move |event: MouseEvent| {
                controller
                    .upgrade()
                    .map(|controller| controller.borrow_mut().pointer_move(event));
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let controller = Rc::downgrade(&self_);
            let presenter = presenter.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                if let Some(controller) = controller.upgrade() {
                    // separate var to break to borrow stack
                    let action_option = controller.borrow_mut().pointer_end(event);
                    if let Some(action) = action_option {
                        action.perform(&presenter);
                    }
                }
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_down = {
            let controller = Rc::downgrade(&self_);
            Closure::wrap(Box::new(move |event: MouseEvent| {
                controller
                    .upgrade()
                    .map(|controller| controller.borrow_mut().pointer_start(event));
            }) as Box<dyn FnMut(_)>)
        };

        let touch_move = {
            let controller = Rc::downgrade(&self_);
            Closure::wrap(Box::new(move |event: TouchEvent| {
                controller
                    .upgrade()
                    .map(|controller| controller.borrow_mut().pointer_move(event));
            }) as Box<dyn FnMut(_)>)
        };
        let touch_end = {
            let controller = Rc::downgrade(&self_);
            let presenter = presenter.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                if let Some(controller) = controller.upgrade() {
                    // separate var to break to borrow stack
                    let action_option = controller.borrow_mut().pointer_end(event);
                    if let Some(action) = action_option {
                        action.perform(&presenter);
                    }
                }
            }) as Box<dyn FnMut(_)>)
        };
        let touch_start = {
            let controller = Rc::downgrade(&self_);
            Closure::wrap(Box::new(move |event: TouchEvent| {
                controller
                    .upgrade()
                    .map(|controller| controller.borrow_mut().pointer_start(event));
            }) as Box<dyn FnMut(_)>)
        };

        view.add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())
            .unwrap();
        view.add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())
            .unwrap();
        view.add_event_listener_with_callback("mousemove", mouse_move.as_ref().unchecked_ref())
            .unwrap();
        view.add_event_listener_with_callback("mouseleave", mouse_up.as_ref().unchecked_ref())
            .unwrap();

        view.add_event_listener_with_callback("touchstart", touch_start.as_ref().unchecked_ref())
            .unwrap();
        view.add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref())
            .unwrap();
        view.add_event_listener_with_callback("touchmove", touch_move.as_ref().unchecked_ref())
            .unwrap();
        view.add_event_listener_with_callback("touchcancel", touch_end.as_ref().unchecked_ref())
            .unwrap();

        mouse_move.forget();
        mouse_up.forget();
        mouse_down.forget();
        touch_move.forget();
        touch_end.forget();
        touch_start.forget();

        // return card as the main view
        view
    }

    fn replace_card(&mut self, card: Card) {
        if let Some(ref old_card) = self.card {
            old_card.card.remove();
        }
        if let Some(ref view) = self.view {
            view.append_with_node_1(&card.card).expect("append_with_node_1 failed");
        } else {
            panic!("view missing");
        }
        self.card = Some(card);
    }

    fn update_card_translation_with_event<T: PointerEvent>(&mut self, event: T) -> i32 {
        if let (Some(pan_start_x), Some(current_x)) = (self.pan_start_x, event.get_x()) {
            let translation = current_x - pan_start_x;
            self.update_card_translation(translation);
            translation
        } else {
            self.translate_x
        }
    }

    fn update_card_translation(&mut self, translation_x: i32) {
        self.translate_x = translation_x;
        if let Some(ref mut card) = self.card {
            card.set_translate(translation_x);
        }
    }

    fn pointer_start<T: PointerEvent>(&mut self, event: T) {
        self.pan_start_x = event.get_x();
        self.translate_x = 0;
    }

    fn pointer_end<T: PointerEvent>(&mut self, event: T) -> Option<PostGestureAction> {
        let mut action = None;
        let translate_x = self.update_card_translation_with_event(event);
        log!("pan ended with translate_x: {}", translate_x);
        if translate_x.abs() > 100 {
            let selection = if translate_x < 0 {
                ChoiceSelection::Left
            } else {
                ChoiceSelection::Right
            };
            self.game.pick(selection);
            if !self.game.in_progress {
                // transition to Menu once the gesture processing is done
                log!("ending game");
                action = Some(PostGestureAction::TransitionApp(State::Menu));
            } else {
                // game is still on -> set new card
                self.replace_card(Card::new(self.game.challenge));
            }
        }
        self.pan_start_x = None;
        self.update_card_translation(0);
        action
    }

    fn pointer_move<T: PointerEvent>(&mut self, event: T) {
        self.update_card_translation_with_event(event);
    }

    fn hide(&mut self) {
        log!("hiding cards");
        if let Some(ref view) = self.view {
            view.remove();
        }
        self.pan_start_x = None;
        self.view = None;
        self.card = None;
    }
}
