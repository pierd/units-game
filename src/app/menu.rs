use super::{App, State, ViewController};
use crate::{log, logic::GameType};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, MouseEvent};

pub struct MenuController {
    app: App,
    view: Option<Element>,
}

impl MenuController {
    pub fn new(app: App) -> Self {
        Self { app, view: None }
    }
}

impl ViewController for MenuController {
    fn show(&mut self) -> Element {
        assert_eq!(self.view, None);

        let document = window().unwrap().document().unwrap();

        // create simple menu
        let view = document.create_element("div").expect("create_element failed");
        view.set_id("menu");
        view.set_class_name("menu");

        let temperature = document.create_element("div").expect("create_element failed");
        temperature.set_class_name("menu-button");
        temperature.set_inner_html("C/F");

        view.append_with_node_1(&temperature)
            .expect("append_with_node_1 failed");

        self.view = Some(view.clone());

        // attach handlers
        let click = {
            let app = self.app.clone();
            Closure::wrap(Box::new(move |_event: MouseEvent| {
                app.transition(State::Playing(GameType::Temperatures));
            }) as Box<dyn FnMut(_)>)
        };
        temperature
            .add_event_listener_with_callback("click", click.as_ref().unchecked_ref())
            .unwrap();
        click.forget();

        view
    }

    fn hide(&mut self) {
        log!("hiding menu");
        if let Some(ref view) = self.view {
            view.remove();
        }
        self.view = None;
    }
}
