use super::gestures::PointerEvent;
use super::{Game, GameType, ViewController};

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, MouseEvent, TouchEvent};

pub struct CardsController {
    controller: Rc<RefCell<CardsControllerImpl>>,
}

impl CardsController {
    pub fn new(game: Game, game_type: GameType) -> Self {
        Self {
            controller: CardsControllerImpl::new(game, game_type),
        }
    }
}

impl ViewController for CardsController {
    fn show(&mut self) -> Element {
        CardsControllerImpl::show(self.controller.clone())
    }

    fn hide(&mut self) {
        self.controller.borrow_mut().hide();
    }
}

struct CardsControllerImpl {
    game: Game,
    view: Option<Element>,

    pan_start_x: Option<i32>,
    card: Option<Element>,
    left: Option<Element>,
    right: Option<Element>,
}

impl CardsControllerImpl {
    fn new(game: Game, _game_type: GameType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            game,
            view: None,
            pan_start_x: None,
            card: None,
            left: None,
            right: None,
        }))
    }

    fn show(self_: Rc<RefCell<Self>>) -> Element {
        let mut controller = self_.borrow_mut();
        assert_eq!(controller.view, None);

        let document = window().unwrap().document().unwrap();

        // create main container for cards
        let view = document
            .create_element("div")
            .expect("create_element failed");
        view.set_class_name("cards");

        // create card view
        let card = document
            .create_element("div")
            .expect("create_element failed");
        card.set_class_name("card");

        // create left side of the card
        let left = document
            .create_element("div")
            .expect("create_element failed");
        left.set_class_name("left");
        left.set_inner_html("30 C");
        card.append_with_node_1(&left)
            .expect("append_with_node_1 failed");
        controller.left = Some(left);

        // create right side of the card
        let right = document
            .create_element("div")
            .expect("create_element failed");
        right.set_class_name("right");
        right.set_inner_html("90 F");
        card.append_with_node_1(&right)
            .expect("append_with_node_1 failed");
        controller.right = Some(right);

        // store the main view and current card
        view.append_with_node_1(&card)
            .expect("append_with_node_1 failed");
        controller.view = Some(view.clone());
        controller.card = Some(card.clone());

        // release the controller borrow
        let _ = controller;

        // attach gestures
        let mouse_move = {
            let controller = self_.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                controller.borrow_mut().pointer_move(event);
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let controller = self_.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                controller.borrow_mut().pointer_end(event);
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_down = {
            let controller = self_.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                controller.borrow_mut().pointer_start(event);
            }) as Box<dyn FnMut(_)>)
        };

        let touch_move = {
            let controller = self_.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                controller.borrow_mut().pointer_move(event);
            }) as Box<dyn FnMut(_)>)
        };
        let touch_end = {
            let controller = self_.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                controller.borrow_mut().pointer_end(event);
            }) as Box<dyn FnMut(_)>)
        };
        let touch_start = {
            let controller = self_.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                controller.borrow_mut().pointer_start(event);
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

    fn set_card_translate(&mut self, translate_x: i32) {
        if let (&mut Some(ref mut card), &mut Some(ref mut left), &mut Some(ref mut right)) =
            (&mut self.card, &mut self.left, &mut self.right)
        {
            card.set_attribute(
                "style",
                &format!(
                    "transform: translate({}px, 0px) rotate({}deg);",
                    translate_x,
                    translate_x as f32 / 10.0
                ),
            )
            .expect("set style failed");

            let scale_adjust = translate_x as f32 / 100.0;
            let left_scale = if scale_adjust > 1.0 {
                0.0
            } else {
                1.0 - scale_adjust
            };
            left.set_attribute(
                "style",
                &format!("transform: scale({}, {});", left_scale, left_scale),
            )
            .expect("set style failed");
            let right_scale = if scale_adjust < -1.0 {
                0.0
            } else {
                1.0 + scale_adjust
            };
            right
                .set_attribute(
                    "style",
                    &format!("transform: scale({}, {});", right_scale, right_scale),
                )
                .expect("set style failed");
        }
    }

    fn pointer_start<T: PointerEvent>(&mut self, event: T) {
        self.pan_start_x = Some(event.get_x());
    }

    fn pointer_end<T: PointerEvent>(&mut self, _event: T) {
        self.pan_start_x = None;
        self.set_card_translate(0);
    }

    fn pointer_move<T: PointerEvent>(&mut self, event: T) {
        if let Some(pan_start_x) = self.pan_start_x {
            self.set_card_translate(event.get_x() - pan_start_x);
        }
    }

    fn hide(&mut self) {
        if let Some(ref view) = self.view {
            view.remove();
        }
        self.pan_start_x = None;
        self.view = None;
        self.card = None;
        self.left = None;
        self.right = None;
    }
}
