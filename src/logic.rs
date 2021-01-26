use std::{collections::HashMap, fmt};

use rand::random;
use rand::seq::IteratorRandom;

#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kilometers,
    NauticalMiles,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Celsius => f.write_str("C"),
            Unit::Fahrenheit => f.write_str("F"),
            Unit::Kilometers => f.write_str("km"),
            Unit::NauticalMiles => f.write_str("NM"),
        }
    }
}

type Float = f32;
type Level = usize;

fn celsius_to_fahrenheit(celsius: Float) -> Float {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: Float) -> Float {
    (fahrenheit - 32.0) / 1.8
}

fn kilometers_to_nauticalmiles(kilometeres: Float) -> Float {
    kilometeres / 1.852
}

fn nauticalmiles_to_kilometers(nauticalmiles: Float) -> Float {
    nauticalmiles * 1.852
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Quantity {
    Temperature,
    Length,
    Area,
    Volume,
    Mass,
    Energy,
    Pressure,
}

#[derive(Clone, Copy, Debug)]
pub struct Challenge {
    pub quantity: Quantity,
    pub left_choice: Choice,
    pub right_choice: Choice,
}

impl Challenge {
    fn is_correct(&self, selection: ChoiceSelection) -> bool {
        match selection {
            ChoiceSelection::Left => self.left_choice.value > self.right_choice.equivalent,
            ChoiceSelection::Right => self.right_choice.value > self.left_choice.equivalent,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Choice {
    pub unit: Unit,
    pub value: Float,
    pub equivalent: Float,
}

#[derive(Debug)]
pub enum ChoiceSelection {
    Left,
    Right,
}

impl Quantity {
    fn generate_temperature_choices(level: Level) -> (Choice, Choice) {
        let c_difference = 50.0 - 5.0 * level as Float;
        let mut c_temperature = -10.0 + 50.0 * random::<Float>();
        let f_higher = random::<bool>();
        let mut f_temperature = celsius_to_fahrenheit(c_temperature + if f_higher { 1.0 } else { -1.0 } * c_difference);
        if f_higher {
            c_temperature = c_temperature.floor();
            f_temperature = f_temperature.ceil();
        } else {
            c_temperature = c_temperature.ceil();
            f_temperature = f_temperature.floor();
        }

        (
            Choice {
                unit: Unit::Celsius,
                value: c_temperature,
                equivalent: celsius_to_fahrenheit(c_temperature),
            },
            Choice {
                unit: Unit::Fahrenheit,
                value: f_temperature,
                equivalent: fahrenheit_to_celsius(f_temperature),
            },
        )
    }

    fn generate_length_choices(level: Level) -> (Choice, Choice) {
        // TODO: based on temperatures but doesn't really work - rething to have just one algo for all quantities
        let km_difference = 1000.0 - 50.0 * level as Float;
        let mut kms = 1000.0 * random::<Float>() + km_difference;
        let nms_higher = random::<bool>();
        let mut nms = kilometers_to_nauticalmiles(kms + if nms_higher { 1.0 } else { -1.0 } * km_difference);
        if nms_higher {
            kms = kms.floor();
            nms = nms.ceil();
        } else {
            kms = kms.ceil();
            nms = nms.floor();
        }

        (
            Choice {
                unit: Unit::Kilometers,
                value: kms,
                equivalent: kilometers_to_nauticalmiles(kms),
            },
            Choice {
                unit: Unit::NauticalMiles,
                value: nms,
                equivalent: nauticalmiles_to_kilometers(nms),
            },
        )
    }

    pub fn generate_challenge(&self, level: Level) -> Challenge {
        let (left_choice, right_choice) = match self {
            Quantity::Temperature => Quantity::generate_temperature_choices(level),
            Quantity::Length => Quantity::generate_length_choices(level),
            _ => Quantity::generate_temperature_choices(level), // FIXME
        };
        if random::<bool>() {
            Challenge {
                quantity: self.clone(),
                left_choice,
                right_choice,
            }
        } else {
            Challenge {
                quantity: self.clone(),
                left_choice: right_choice,
                right_choice: left_choice,
            }
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub in_progress: bool,
    pub level_per_quantity: HashMap<Quantity, Level>,
    pub challenge: Challenge,
}

impl Game {
    pub fn new_with_single_quantity(quantity: Quantity) -> Self {
        let mut level_per_quantity = HashMap::with_capacity(1);
        level_per_quantity.insert(quantity, 0);
        Self {
            in_progress: true,
            level_per_quantity,
            challenge: quantity.generate_challenge(0),
        }
    }

    pub fn pick(&mut self, selection: ChoiceSelection) {
        if self.challenge.is_correct(selection) {
            self.level_per_quantity
                .entry(self.challenge.quantity)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            let mut rng = rand::thread_rng();
            let next_quantity = self.level_per_quantity.keys().choose(&mut rng).unwrap();
            let level = self.level_per_quantity.get(next_quantity).unwrap_or(&0).clone();
            self.challenge = next_quantity.generate_challenge(level);
        } else {
            self.in_progress = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats_close_enough(a: Float, b: Float) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn temperature_conversions_work() {
        assert_eq!(celsius_to_fahrenheit(-40.0), fahrenheit_to_celsius(-40.0));
        assert!(floats_close_enough(celsius_to_fahrenheit(0.0), 32.0));
        assert!(floats_close_enough(celsius_to_fahrenheit(100.0), 212.0));
        assert!(floats_close_enough(fahrenheit_to_celsius(0.0), -17.777777));
        assert!(floats_close_enough(fahrenheit_to_celsius(100.0), 37.777777));
    }

    #[test]
    fn length_conversions_work() {
        assert_eq!(kilometers_to_nauticalmiles(0.0), nauticalmiles_to_kilometers(0.0));
        assert!(floats_close_enough(nauticalmiles_to_kilometers(1000.0), 1852.0));
    }

    #[test]
    fn new_game_is_in_progress() {
        assert_eq!(Game::new_with_single_quantity(Quantity::Temperature).in_progress, true);
    }

    #[test]
    fn correct_pick_increases_level() {
        let mut level_per_quantity = HashMap::new();
        level_per_quantity.insert(Quantity::Temperature, 3);
        let mut game = Game {
            in_progress: true,
            level_per_quantity,
            challenge: Challenge {
                quantity: Quantity::Temperature,
                left_choice: Choice {
                    unit: Unit::Celsius,
                    value: 30.0,
                    equivalent: 30.0,
                },
                right_choice: Choice {
                    unit: Unit::Celsius,
                    value: 10.0,
                    equivalent: 10.0,
                },
            },
        };
        game.pick(ChoiceSelection::Left);
        assert_eq!(game.level_per_quantity.get(&Quantity::Temperature), Some(&4));
        assert_eq!(game.in_progress, true);
    }

    #[test]
    fn wrong_pick_stops_game() {
        let mut level_per_quantity = HashMap::new();
        level_per_quantity.insert(Quantity::Temperature, 3);
        let mut game = Game {
            in_progress: true,
            level_per_quantity,
            challenge: Challenge {
                quantity: Quantity::Temperature,
                left_choice: Choice {
                    unit: Unit::Celsius,
                    value: 30.0,
                    equivalent: 30.0,
                },
                right_choice: Choice {
                    unit: Unit::Celsius,
                    value: 10.0,
                    equivalent: 10.0,
                },
            },
        };
        game.pick(ChoiceSelection::Right);
        assert_eq!(game.level_per_quantity.get(&Quantity::Temperature), Some(&3));
        assert_eq!(game.in_progress, false);
    }
}
