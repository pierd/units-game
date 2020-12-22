use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{window, Element, MouseEvent, TouchEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        #[allow(unused_unsafe)]
        unsafe {
            log(&format!($($arg)*));
        }
    }}
}

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

struct Game {
    content: Element,
    pan_start_x: Option<i32>,
    card: Option<Element>,
    left: Option<Element>,
    right: Option<Element>,
}

impl Game {
    fn new(content: Element) -> Self {
        Self {
            content,
            pan_start_x: None,
            card: None,
            left: None,
            right: None,
        }
    }

    fn add_card(&mut self) {
        let document = window().unwrap().document().unwrap();
        let card = document.create_element("div").expect("create_element failed");
        card.set_id("test");
        card.set_class_name("card");

        let left = document.create_element("div").expect("create_element failed");
        left.set_class_name("left");
        left.set_inner_html("30 C");
        card.append_with_node_1(&left).expect("append_with_node_1 failed");
        self.left = Some(left);

        let right = document.create_element("div").expect("create_element failed");
        right.set_class_name("right");
        right.set_inner_html("90 F");
        card.append_with_node_1(&right).expect("append_with_node_1 failed");
        self.right = Some(right);

        self.content.append_with_node_1(&card).expect("append_with_node_1 failed");
        self.card = Some(card);
    }

    fn set_card_translate(&mut self, translate_x: i32) {
        if let (&mut Some(ref mut card), &mut Some(ref mut left), &mut Some(ref mut right)) = (&mut self.card, &mut self.left, &mut self.right) {
            card.set_attribute(
                "style",
                &format!("transform: translate({}px, 0px) rotate({}deg);", translate_x, translate_x as f32/ 10.0)
            ).expect("set style failed");

            let scale_adjust = translate_x as f32 / 100.0;
            let left_scale = if scale_adjust > 1.0 {
                0.0
            } else {
                1.0 - scale_adjust
            };
            left.set_attribute(
                "style",
                &format!("transform: scale({}, {});", left_scale, left_scale)
            ).expect("set style failed");
            let right_scale = if scale_adjust < -1.0 {
                0.0
            } else {
                1.0 + scale_adjust
            };
            right.set_attribute(
                "style",
                &format!("transform: scale({}, {});", right_scale, right_scale)
            ).expect("set style failed");
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
            game: Rc::new(RefCell::new(Game::new(element.clone()))),
            content: element,
        }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log!("Starting in: {}", self.content.id());
        self.game.borrow_mut().add_card();

        let mouse_move = {
            let game = self.game.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                game.borrow_mut().pointer_move(event);
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let game = self.game.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                game.borrow_mut().pointer_end(event);
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_down = {
            let game = self.game.clone();
            Closure::wrap(Box::new(move |event: MouseEvent| {
                game.borrow_mut().pointer_start(event);
            }) as Box<dyn FnMut(_)>)
        };

        let touch_move = {
            let game = self.game.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                game.borrow_mut().pointer_move(event);
            }) as Box<dyn FnMut(_)>)
        };
        let touch_end = {
            let game = self.game.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                game.borrow_mut().pointer_end(event);
            }) as Box<dyn FnMut(_)>)
        };
        let touch_start = {
            let game = self.game.clone();
            Closure::wrap(Box::new(move |event: TouchEvent| {
                game.borrow_mut().pointer_start(event);
            }) as Box<dyn FnMut(_)>)
        };

        self.content
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())
            .unwrap();
        self.content
            .add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())
            .unwrap();
        self.content
            .add_event_listener_with_callback("mousemove", mouse_move.as_ref().unchecked_ref())
            .unwrap();
        self.content
            .add_event_listener_with_callback("mouseleave", mouse_up.as_ref().unchecked_ref())
            .unwrap();

        self.content
            .add_event_listener_with_callback("touchstart", touch_start.as_ref().unchecked_ref())
            .unwrap();
        self.content
            .add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref())
            .unwrap();
        self.content
            .add_event_listener_with_callback("touchmove", touch_move.as_ref().unchecked_ref())
            .unwrap();
        self.content
            .add_event_listener_with_callback("touchcancel", touch_end.as_ref().unchecked_ref())
            .unwrap();

        mouse_move.forget();
        mouse_up.forget();
        mouse_down.forget();
        touch_move.forget();
        touch_end.forget();
        touch_start.forget();

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
