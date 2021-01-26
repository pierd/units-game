use super::gestures::PointerEvent;
use super::{log, Presenter, Reaction, State, ViewController};
use crate::logic::{Challenge, ChoiceSelection, Game, Quantity};

use web_sys::{window, Element, MouseEvent, TouchEvent};

struct Card {
    card: Element,
    left: Element,
    right: Element,
}

impl Card {
    fn new(challenge: Challenge) -> Self {
        let document = window().unwrap().document().unwrap();

        // create card view
        let card = document.create_element("div").expect("create_element failed");
        card.set_class_name("card");

        // create left side of the card
        let left_choice = challenge.left_choice;
        let left = document.create_element("div").expect("create_element failed");
        left.set_class_name("left");
        left.set_inner_html(&format!("{} {}", left_choice.value, left_choice.unit));
        card.append_with_node_1(&left).expect("append_with_node_1 failed");

        // create right side of the card
        let right_choice = challenge.right_choice;
        let right = document.create_element("div").expect("create_element failed");
        right.set_class_name("right");
        right.set_inner_html(&format!("{} {}", right_choice.value, right_choice.unit));
        card.append_with_node_1(&right).expect("append_with_node_1 failed");

        Self { card, left, right }
    }

    fn set_translate(&mut self, translate_x: i32) {
        self.card
            .set_attribute(
                "style",
                &format!(
                    "transform: translate({}px, 0px) rotate({}deg);",
                    translate_x,
                    translate_x as f32 / 10.0
                ),
            )
            .expect("set style failed");

        let scale_adjust = translate_x as f32 / 100.0;
        let left_scale = if scale_adjust > 1.0 { 0.0 } else { 1.0 - scale_adjust };
        self.left
            .set_attribute("style", &format!("transform: scale({}, {});", left_scale, left_scale))
            .expect("set style failed");
        let right_scale = if scale_adjust < -1.0 { 0.0 } else { 1.0 + scale_adjust };
        self.right
            .set_attribute("style", &format!("transform: scale({}, {});", right_scale, right_scale))
            .expect("set style failed");
    }
}

pub struct CardsController {
    game: Game,
    view: Option<Element>,

    pan_start_x: Option<i32>,
    translate_x: i32,
    card: Option<Card>,
}

impl CardsController {
    pub fn new(quantity: Quantity) -> Self {
        Self {
            game: Game::new_with_single_quantity(quantity),
            view: None,
            pan_start_x: None,
            translate_x: 0,
            card: None,
        }
    }

    fn replace_card(&mut self, card: Card) {
        if let Some(ref old_card) = self.card {
            old_card.card.remove();
        }
        if let Some(ref view) = self.view {
            view.append_with_node_1(&card.card).expect("append_with_node_1 failed");
        } else {
            panic!("view missing");
        }
        self.card = Some(card);
    }

    fn update_card_translation_with_event<T: PointerEvent>(&mut self, event: T) -> i32 {
        if let (Some(pan_start_x), Some(current_x)) = (self.pan_start_x, event.get_x()) {
            let translation = current_x - pan_start_x;
            self.update_card_translation(translation);
            translation
        } else {
            self.translate_x
        }
    }

    fn update_card_translation(&mut self, translation_x: i32) {
        self.translate_x = translation_x;
        if let Some(ref mut card) = self.card {
            card.set_translate(translation_x);
        }
    }

    fn pointer_start<T: PointerEvent>(&mut self, event: T) -> Option<Reaction> {
        self.pan_start_x = event.get_x();
        self.translate_x = 0;
        None
    }

    fn pointer_end<T: PointerEvent>(&mut self, event: T) -> Option<Reaction> {
        let mut reaction = None;
        let translate_x = self.update_card_translation_with_event(event);
        log!("pan ended with translate_x: {}", translate_x);
        if translate_x.abs() > 100 {
            let selection = if translate_x < 0 {
                ChoiceSelection::Left
            } else {
                ChoiceSelection::Right
            };
            self.game.pick(selection);
            if !self.game.in_progress {
                // transition to Menu once the gesture processing is done
                log!("ending game");
                reaction = Some(Reaction::Transition(State::Menu));
            } else {
                // game is still on -> set new card
                self.replace_card(Card::new(self.game.challenge));
            }
        }
        self.pan_start_x = None;
        self.update_card_translation(0);
        reaction
    }

    fn pointer_move<T: PointerEvent>(&mut self, event: T) -> Option<Reaction> {
        self.update_card_translation_with_event(event);
        None
    }
}

impl ViewController for CardsController {
    fn show(&mut self, mut presenter: Presenter) -> Element {
        assert_eq!(self.view, None);

        let document = window().unwrap().document().unwrap();

        // create main container for cards
        let view = document.create_element("div").expect("create_element failed");
        view.set_class_name("cards");
        self.view = Some(view.clone());

        // create card and add it to the view
        let card = Card::new(self.game.challenge);
        self.replace_card(card);

        // attach gestures
        presenter.add_event_listener(&view, "mousedown", CardsController::pointer_start::<MouseEvent>);
        presenter.add_event_listener(&view, "mouseup", CardsController::pointer_end::<MouseEvent>);
        presenter.add_event_listener(&view, "mouseleave", CardsController::pointer_end::<MouseEvent>);
        presenter.add_event_listener(&view, "mousemove", CardsController::pointer_move::<MouseEvent>);
        presenter.add_event_listener(&view, "touchstart", CardsController::pointer_start::<TouchEvent>);
        presenter.add_event_listener(&view, "touchend", CardsController::pointer_end::<TouchEvent>);
        presenter.add_event_listener(&view, "touchcancel", CardsController::pointer_end::<TouchEvent>);
        presenter.add_event_listener(&view, "touchmove", CardsController::pointer_move::<TouchEvent>);

        // return card as the main view
        view
    }

    fn hide(&mut self) {
        if let Some(ref view) = self.view {
            view.remove();
        }
        self.pan_start_x = None;
        self.view = None;
        self.card = None;
    }
}
