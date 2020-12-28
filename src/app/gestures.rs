use web_sys::{MouseEvent, TouchEvent};

pub trait PointerEvent {
    fn get_x(&self) -> Option<i32>;
}

impl PointerEvent for MouseEvent {
    fn get_x(&self) -> Option<i32> {
        Some(self.client_x())
    }
}

impl PointerEvent for TouchEvent {
    fn get_x(&self) -> Option<i32> {
        self.touches().item(0).map(|touch| touch.client_x())
    }
}
