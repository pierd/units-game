use super::{log, Presenter, Reaction, State, ViewController};
use crate::logic::GameType;

use web_sys::{window, Document, Element};
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
        self.view = Some(view.clone());

        create_unit_button(&mut presenter, &document, &view, "C/F", GameType::Temperature);
        create_unit_button(&mut presenter, &document, &view, "km/M", GameType::Length);
        create_unit_button(&mut presenter, &document, &view, "m^2/???", GameType::Area);
        create_unit_button(&mut presenter, &document, &view, "L/oz", GameType::Volume);
        create_unit_button(&mut presenter, &document, &view, "kg,lbs", GameType::Mass);

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

fn create_unit_button(
    presenter: &mut Presenter,
    document: &Document,
    parent: &Element,
    inner_html: &str,
    game_type: GameType,
) {
    let button = document.create_element("div").expect("create_element failed");
    button.set_class_name("menu-button");
    button.set_inner_html(inner_html);
    parent.append_with_node_1(&button).expect("append_with_node_1 failed");
    // attach handlers
    presenter.add_event_reaction(&button, "click", Reaction::Transition(State::Playing(game_type)));
}
