use crate::logic::Quantity;

use super::{log, Presenter, Reaction, State, ViewController};

use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlImageElement};

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

        create_unit_button(&mut presenter, &document, &view, "C/F", Quantity::Temperature);
        create_unit_button(&mut presenter, &document, &view, "km/M", Quantity::Length);
        create_unit_button(&mut presenter, &document, &view, "m^2/sq ft", Quantity::Area);
        create_unit_button(&mut presenter, &document, &view, "L/fl oz", Quantity::Volume);
        create_unit_button(&mut presenter, &document, &view, "kg/lbs", Quantity::Mass);
        create_unit_button(&mut presenter, &document, &view, "cal/J", Quantity::Energy);
        create_unit_button(&mut presenter, &document, &view, "psi/kPa", Quantity::Pressure);
        create_unit_button(&mut presenter, &document, &view, "all", Quantity::Pressure);    // FIXME

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
    quantity: Quantity,
) {
    let icon: HtmlImageElement = document.create_element("img").expect("create_element failed").dyn_into().expect("cast failed");
    icon.set_src("assets/icon-placeholder.png");

    let button = document.create_element("div").expect("create_element failed");
    button.set_class_name("menu-button");
    button.set_inner_html(inner_html);

    // create hierarchy
    button.append_with_node_1(&icon).expect("append_with_node_1 failed");
    parent.append_with_node_1(&button).expect("append_with_node_1 failed");

    // attach handlers
    presenter.add_event_reaction(&button, "click", Reaction::Transition(State::Playing(quantity)));
}
