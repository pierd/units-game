use crate::logic::Quantity;

use super::{log, Presenter, Reaction, State, ViewController};

use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlImageElement};

const MENU_RADIUS_VH: f32 = 20.0;
const QUANTITIES: [Quantity; 7] = [
    Quantity::Temperature,
    Quantity::Length,
    Quantity::Area,
    Quantity::Volume,
    Quantity::Mass,
    Quantity::Energy,
    Quantity::Pressure,
];

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

        Button::new_button(
            &mut presenter,
            &document,
            &view,
            "assets/temperature.svg", // FIXME
            "settings",
            Reaction::Transition(State::Settings),
        );

        let buttons_in_circle = QUANTITIES.len() + 1;
        for (i, quantity) in QUANTITIES.iter().enumerate() {
            Button::new_unit_button(
                &mut presenter,
                &document,
                &view,
                quantity_to_string(*quantity),
                *quantity,
            )
            .place_in_circle(MENU_RADIUS_VH, i, buttons_in_circle);
        }
        Button::new_all_units_button(&mut presenter, &document, &view, "all").place_in_circle(
            MENU_RADIUS_VH,
            7,
            buttons_in_circle,
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

fn quantity_to_string(quantity: Quantity) -> &'static str {
    match quantity {
        Quantity::Temperature => "C/F",
        Quantity::Length => "km/M",
        Quantity::Area => "m^2/sq ft",
        Quantity::Volume => "L/fl oz",
        Quantity::Mass => "kg/lbs",
        Quantity::Energy => "cal/J",
        Quantity::Pressure => "psi/kPa",
    }
}

fn quantity_to_icon_src(_quantity: Quantity) -> String {
    // FIXME: uncomment once the rest of the assets are available
    // format!("assets/{}.svg", quantity)
    "assets/temperature.svg".to_owned()
}

struct Button {
    button: Element,
    icon: HtmlImageElement,
}

impl Button {
    fn new_button(
        presenter: &mut Presenter,
        document: &Document,
        parent: &Element,
        icon_src: &str,
        inner_html: &str,
        reaction: Reaction,
    ) -> Self {
        let icon: HtmlImageElement = document
            .create_element("img")
            .expect("create_element failed")
            .dyn_into()
            .expect("cast failed");
        icon.set_src(icon_src);

        let button = document.create_element("div").expect("create_element failed");
        button.set_class_name("menu-button");
        // button.set_inner_html(inner_html);

        // create hierarchy
        button.append_with_node_1(&icon).expect("append_with_node_1 failed");
        parent.append_with_node_1(&button).expect("append_with_node_1 failed");

        // attach handlers
        presenter.add_event_reaction(&button, "click", reaction);

        Self { button, icon }
    }

    fn new_unit_button(
        presenter: &mut Presenter,
        document: &Document,
        parent: &Element,
        inner_html: &str,
        quantity: Quantity,
    ) -> Self {
        Self::new_button(
            presenter,
            document,
            parent,
            &quantity_to_icon_src(quantity),
            inner_html,
            Reaction::Transition(State::Playing(quantity)),
        )
    }

    fn new_all_units_button(
        presenter: &mut Presenter,
        document: &Document,
        parent: &Element,
        inner_html: &str,
    ) -> Self {
        Self::new_button(
            presenter,
            document,
            parent,
            "assets/temperature.svg",
            inner_html,
            Reaction::Transition(State::Playing(Quantity::Pressure)),
        )
    }

    fn place_in_circle(&self, radius_vh: f32, idx: usize, count: usize) {
        let rotation = 360.0 / count as f32 * idx as f32;
        self.button
            .set_attribute(
                "style",
                &format!("transform: rotate({}deg) translateX({}vh);", rotation, -radius_vh),
            )
            .expect("set style failed");
        self.icon
            .set_attribute("style", &format!("transform: rotate({}deg);", -rotation))
            .expect("set style failed");
    }
}
