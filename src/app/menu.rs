use super::{Presenter, Reaction, State, ViewController};
use crate::{log, logic::GameType};

use web_sys::{window, Element};
pub struct MenuController {
    view: Option<Element>,
}

impl Default for MenuController {
    fn default() -> Self {
        Self { view: None }
    }
}

impl ViewController for MenuController {
    fn show(&mut self, mut presenter: Presenter) -> Element {
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
        presenter.add_event_reaction(
            &temperature,
            "click",
            Reaction::Transition(State::Playing(GameType::Temperatures)),
        );

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
