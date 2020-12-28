use super::log;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, MouseEvent, TouchEvent};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    None,
    Menu,
    Playing,
}

pub(crate) struct Game {
    content: Element,
    state: State,
    pan_start_x: Option<i32>,
    card: Option<Element>,
    left: Option<Element>,
    right: Option<Element>,
}

impl Game {
    pub fn new(content: Element) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            content,
            state: State::None,
            pan_start_x: None,
            card: None,
            left: None,
            right: None,
        }))
    }

    pub fn run(self_: Rc<RefCell<Self>>) {
        {
            let mut game = self_.borrow_mut();
            assert!(game.state == State::None);
            game.state = State::Menu;
        }
        Self::show_menu(self_);
    }

    fn show_menu(self_: Rc<RefCell<Self>>) {
        log!("would show menu");
        Self::start(self_);
    }

    fn start(self_: Rc<RefCell<Self>>) {
        let mut game = self_.borrow_mut();
        game.add_card();

        let mouse_move = {
            let game = self_.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                game.borrow_mut().pointer_move(event);
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let game = self_.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                game.borrow_mut().pointer_end(event);
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_down = {
            let game = self_.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                game.borrow_mut().pointer_start(event);
            }) as Box<dyn FnMut(_)>)
        };

        let touch_move = {
            let game = self_.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                game.borrow_mut().pointer_move(event);
            }) as Box<dyn FnMut(_)>)
        };
        let touch_end = {
            let game = self_.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                game.borrow_mut().pointer_end(event);
            }) as Box<dyn FnMut(_)>)
        };
        let touch_start = {
            let game = self_.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                game.borrow_mut().pointer_start(event);
            }) as Box<dyn FnMut(_)>)
        };

        game.content
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())
            .unwrap();
        game.content
            .add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())
            .unwrap();
        game.content
            .add_event_listener_with_callback("mousemove", mouse_move.as_ref().unchecked_ref())
            .unwrap();
        game.content
            .add_event_listener_with_callback("mouseleave", mouse_up.as_ref().unchecked_ref())
            .unwrap();

        game.content
            .add_event_listener_with_callback("touchstart", touch_start.as_ref().unchecked_ref())
            .unwrap();
        game.content
            .add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref())
            .unwrap();
        game.content
            .add_event_listener_with_callback("touchmove", touch_move.as_ref().unchecked_ref())
            .unwrap();
        game.content
            .add_event_listener_with_callback("touchcancel", touch_end.as_ref().unchecked_ref())
            .unwrap();

        mouse_move.forget();
        mouse_up.forget();
        mouse_down.forget();
        touch_move.forget();
        touch_end.forget();
        touch_start.forget();
    }

    fn add_card(&mut self) {
        let document = window().unwrap().document().unwrap();
        let card = document
            .create_element("div")
            .expect("create_element failed");
        card.set_id("test");
        card.set_class_name("card");

        let left = document
            .create_element("div")
            .expect("create_element failed");
        left.set_class_name("left");
        left.set_inner_html("30 C");
        card.append_with_node_1(&left)
            .expect("append_with_node_1 failed");
        self.left = Some(left);

        let right = document
            .create_element("div")
            .expect("create_element failed");
        right.set_class_name("right");
        right.set_inner_html("90 F");
        card.append_with_node_1(&right)
            .expect("append_with_node_1 failed");
        self.right = Some(right);

        self.content
            .append_with_node_1(&card)
            .expect("append_with_node_1 failed");
        self.card = Some(card);
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
