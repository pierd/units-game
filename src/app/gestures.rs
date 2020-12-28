use web_sys::{MouseEvent, TouchEvent};

pub trait PointerEvent {
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
