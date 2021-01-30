use std::{collections::HashMap, fmt};

use rand::random;
use rand::seq::IteratorRandom;

type Float = f32;
type Level = usize;

/// Note: Units are ordered by their relative delta. That is a difference of a Fahrenheit degree is smaller than
/// a difference of a Celsius degree or a foot is smaller than a meter and so on.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Unit {
    // Temperature
    Fahrenheit,
    Celsius,

    // Length
    Foot,
    Meter,
    Kilometer,
    Mile,
    NauticalMile,

    // Volume
    Millilitre,
    FluidOunce, // US, not imperial
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Fahrenheit => f.write_str("F"),
            Unit::Celsius => f.write_str("C"),
            Unit::Foot => f.write_str("ft"),
            Unit::Meter => f.write_str("m"),
            Unit::Kilometer => f.write_str("km"),
            Unit::Mile => f.write_str("mi"),
            Unit::NauticalMile => f.write_str("NM"),
            Unit::Millilitre => f.write_str("ml"),
            Unit::FluidOunce => f.write_str("fl oz"),
        }
    }
}

fn convert(value: Float, from: Unit, to: Unit) -> Float {
    match (from, to) {
        (x, y) if x == y => value,

        (Unit::Celsius, Unit::Fahrenheit) => value * 1.8 + 32.0,
        (Unit::Fahrenheit, Unit::Celsius) => (value - 32.0) / 1.8,

        (Unit::Meter, Unit::Foot) => value / 0.3048,
        (Unit::Foot, Unit::Meter) => value * 0.3048,

        (Unit::Kilometer, Unit::NauticalMile) => value / 1.852,
        (Unit::NauticalMile, Unit::Kilometer) => value * 1.852,
        (Unit::Mile, Unit::Kilometer) => value / 1.609344,
        (Unit::Kilometer, Unit::Mile) => value * 1.609344,
        (Unit::Mile, Unit::NauticalMile) => convert(
            convert(value, Unit::Mile, Unit::Kilometer),
            Unit::Kilometer,
            Unit::NauticalMile,
        ),
        (Unit::NauticalMile, Unit::Mile) => convert(
            convert(value, Unit::NauticalMile, Unit::Kilometer),
            Unit::Kilometer,
            Unit::Mile,
        ),

        (Unit::Millilitre, Unit::FluidOunce) => value / 29.5735295625,
        (Unit::FluidOunce, Unit::Millilitre) => value * 29.5735295625,

        _ => panic!("can't convert from {:?} to {:?}", from, to),
    }
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
    pub fn unit_groups(&self) -> &[&[Unit]] {
        match self {
            Quantity::Temperature => &[&[Unit::Celsius, Unit::Fahrenheit]],
            Quantity::Length => &[
                &[Unit::Meter, Unit::Foot],
                &[Unit::Kilometer, Unit::NauticalMile, Unit::Mile],
            ],
            Quantity::Area => &[],
            Quantity::Volume => &[&[Unit::Millilitre, Unit::FluidOunce]],
            Quantity::Mass => &[],
            Quantity::Energy => &[],
            Quantity::Pressure => &[],
        }
    }

    fn generate_temperature_choices(level: Level) -> (Choice, Choice) {
        let c_difference = 50.0 - 5.0 * level as Float;
        let mut c_temperature = -10.0 + 50.0 * random::<Float>();
        let f_higher = random::<bool>();
        let mut f_temperature = convert(
            c_temperature + if f_higher { 1.0 } else { -1.0 } * c_difference,
            Unit::Celsius,
            Unit::Fahrenheit,
        );
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
                equivalent: convert(c_temperature, Unit::Celsius, Unit::Fahrenheit),
            },
            Choice {
                unit: Unit::Fahrenheit,
                value: f_temperature,
                equivalent: convert(f_temperature, Unit::Fahrenheit, Unit::Celsius),
            },
        )
    }

    fn generate_length_choices(level: Level) -> (Choice, Choice) {
        // TODO: based on temperatures but doesn't really work - rething to have just one algo for all quantities
        let km_difference = 1000.0 - 50.0 * level as Float;
        let mut kms = 1000.0 * random::<Float>() + km_difference;
        let nms_higher = random::<bool>();
        let mut nms = convert(
            kms + if nms_higher { 1.0 } else { -1.0 } * km_difference,
            Unit::Kilometer,
            Unit::NauticalMile,
        );
        if nms_higher {
            kms = kms.floor();
            nms = nms.ceil();
        } else {
            kms = kms.ceil();
            nms = nms.floor();
        }

        (
            Choice {
                unit: Unit::Kilometer,
                value: kms,
                equivalent: convert(kms, Unit::Kilometer, Unit::NauticalMile),
            },
            Choice {
                unit: Unit::NauticalMile,
                value: nms,
                equivalent: convert(nms, Unit::NauticalMile, Unit::Kilometer),
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

    // TODO: generate challenge based on unit_group, not on quantity
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
    fn conversions_work() {
        assert_eq!(
            convert(-40.0, Unit::Celsius, Unit::Fahrenheit),
            convert(-40.0, Unit::Fahrenheit, Unit::Celsius)
        );
        assert!(floats_close_enough(convert(0.0, Unit::Celsius, Unit::Celsius), 0.0));
        assert!(floats_close_enough(convert(0.0, Unit::Celsius, Unit::Fahrenheit), 32.0));
        assert!(floats_close_enough(
            convert(100.0, Unit::Celsius, Unit::Fahrenheit),
            212.0
        ));
        assert!(floats_close_enough(
            convert(0.0, Unit::Fahrenheit, Unit::Celsius),
            -17.777777
        ));
        assert!(floats_close_enough(
            convert(100.0, Unit::Fahrenheit, Unit::Celsius),
            37.777777
        ));

        assert_eq!(
            convert(0.0, Unit::Kilometer, Unit::NauticalMile),
            convert(0.0, Unit::NauticalMile, Unit::Kilometer)
        );
        assert!(floats_close_enough(
            convert(1000.0, Unit::NauticalMile, Unit::Kilometer),
            1852.0
        ));
    }

    #[test]
    fn units_ordered_properly() {
        assert!(Unit::Fahrenheit < Unit::Celsius);
        assert!(Unit::Foot < Unit::Meter);
        assert_eq!(
            [Unit::Mile, Unit::Kilometer, Unit::NauticalMile].iter().min(),
            Some(&Unit::Kilometer)
        );
        assert!(Unit::FluidOunce > Unit::Millilitre);
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
