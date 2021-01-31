use std::{collections::HashMap, fmt};

use rand::random;
use rand::seq::IteratorRandom;

type Float = f32;
type Level = usize;

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

/// Note: Units are ordered by their relative delta. That is a difference of a Fahrenheit degree is smaller than
/// a difference of a Celsius degree or a foot is smaller than a meter and so on.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    // Area
    SquareFoot,
    SquareMeter,
    Acre,
    Hectare,

    // Volume
    Millilitre,
    FluidOunce, // US
    Litre,
    Gallon, // US

    // Mass
    Pound,
    Kilogram,

    // Energy
    Joule,
    Calorie,

    // Pressure
    Kilopascal,
    PoundPerSquareInch,
}

#[cfg(test)]
const ALL_UNITS: &[Unit] = &[
    // Temperature
    Unit::Fahrenheit,
    Unit::Celsius,
    // Length
    Unit::Foot,
    Unit::Meter,
    Unit::Kilometer,
    Unit::Mile,
    Unit::NauticalMile,
    // Area
    Unit::SquareFoot,
    Unit::SquareMeter,
    Unit::Acre,
    Unit::Hectare,
    // Volume
    Unit::Millilitre,
    Unit::FluidOunce,
    Unit::Litre,
    Unit::Gallon,
    // Mass
    Unit::Pound,
    Unit::Kilogram,
    // Energy
    Unit::Joule,
    Unit::Calorie,
    // Pressure
    Unit::Kilopascal,
    Unit::PoundPerSquareInch,
];

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

            Unit::SquareFoot => f.write_str("sq ft"),
            Unit::SquareMeter => f.write_str("m^2"),
            Unit::Acre => f.write_str("acre"),
            Unit::Hectare => f.write_str("ha"),

            Unit::Millilitre => f.write_str("mL"),
            Unit::FluidOunce => f.write_str("fl oz"),
            Unit::Litre => f.write_str("L"),
            Unit::Gallon => f.write_str("gal"),

            Unit::Pound => f.write_str("lb"),
            Unit::Kilogram => f.write_str("kg"),

            Unit::Joule => f.write_str("J"),
            Unit::Calorie => f.write_str("cal"),

            Unit::Kilopascal => f.write_str("kPa"),
            Unit::PoundPerSquareInch => f.write_str("psi"),
        }
    }
}

impl Unit {
    fn pair_with(self, other: Self) -> (Self, Self) {
        if self < other {
            (self, other)
        } else {
            (other, self)
        }
    }

    fn level0_delta(&self) -> Float {
        (self.max_value() - self.min_value()) / 2.1 // slightly less than half
    }

    fn level_delta(&self, level: Level) -> Float {
        (self.level0_delta() * (0.9 as Float).powi(level as i32)).max(1.0)
    }

    fn min_value(&self) -> Float {
        match self {
            Unit::Fahrenheit => convert(Unit::Celsius.min_value(), Unit::Celsius, Unit::Fahrenheit).unwrap(),
            Unit::Celsius => -40.0,
            _ => 1.0,
        }
    }

    fn max_value(&self) -> Float {
        match self {
            Unit::Celsius => 50.0,
            Unit::Foot => 200.0,
            Unit::Gallon => 99.0,
            Unit::Pound => 500.0,
            Unit::Kilopascal => 200.0,

            Unit::Fahrenheit => convert(Unit::Celsius.max_value(), Unit::Celsius, Unit::Fahrenheit).unwrap(),
            Unit::Meter => convert(Unit::Foot.max_value(), Unit::Foot, Unit::Meter).unwrap(),
            Unit::Mile => convert(Unit::Kilometer.max_value(), Unit::Kilometer, Unit::Mile).unwrap(),
            Unit::NauticalMile => convert(Unit::Kilometer.max_value(), Unit::Kilometer, Unit::Mile).unwrap(),
            Unit::SquareMeter => convert(Unit::SquareFoot.max_value(), Unit::SquareFoot, Unit::SquareMeter).unwrap(),
            Unit::Hectare => convert(Unit::Acre.max_value(), Unit::Acre, Unit::Hectare).unwrap(),
            Unit::FluidOunce => convert(Unit::Millilitre.max_value(), Unit::Millilitre, Unit::FluidOunce).unwrap(),
            Unit::Litre => convert(Unit::Gallon.max_value(), Unit::Gallon, Unit::Litre).unwrap(),
            Unit::Kilogram => convert(Unit::Pound.max_value(), Unit::Pound, Unit::Kilogram).unwrap(),
            Unit::Calorie => convert(Unit::Joule.max_value(), Unit::Joule, Unit::Calorie).unwrap(),
            Unit::PoundPerSquareInch => {
                convert(Unit::Kilopascal.max_value(), Unit::Kilopascal, Unit::PoundPerSquareInch).unwrap()
            }

            _ => 999.0,
        }
    }
}

fn convert(value: Float, from: Unit, to: Unit) -> Option<Float> {
    match (from, to) {
        (x, y) if x == y => Some(value),

        (Unit::Celsius, Unit::Fahrenheit) => Some(value * 1.8 + 32.0),
        (Unit::Fahrenheit, Unit::Celsius) => Some((value - 32.0) / 1.8),

        (Unit::Meter, Unit::Foot) => Some(value / 0.3048),
        (Unit::Foot, Unit::Meter) => Some(value * 0.3048),

        (Unit::Kilometer, Unit::NauticalMile) => Some(value / 1.852),
        (Unit::NauticalMile, Unit::Kilometer) => Some(value * 1.852),
        (Unit::Mile, Unit::Kilometer) => Some(value * 1.609344),
        (Unit::Kilometer, Unit::Mile) => Some(value / 1.609344),
        (Unit::Mile, Unit::NauticalMile) => convert(
            convert(value, Unit::Mile, Unit::Kilometer).unwrap(),
            Unit::Kilometer,
            Unit::NauticalMile,
        ),
        (Unit::NauticalMile, Unit::Mile) => convert(
            convert(value, Unit::NauticalMile, Unit::Kilometer).unwrap(),
            Unit::Kilometer,
            Unit::Mile,
        ),

        (Unit::SquareFoot, Unit::SquareMeter) => Some(value * 0.09290341),
        (Unit::SquareMeter, Unit::SquareFoot) => Some(value / 0.09290341),

        (Unit::Hectare, Unit::Acre) => Some(value * 2.4711),
        (Unit::Acre, Unit::Hectare) => Some(value / 2.4711),

        (Unit::Millilitre, Unit::FluidOunce) => Some(value / 29.5735295625),
        (Unit::FluidOunce, Unit::Millilitre) => Some(value * 29.5735295625),

        (Unit::Gallon, Unit::Litre) => Some(value * 3.785411784),
        (Unit::Litre, Unit::Gallon) => Some(value / 3.785411784),

        (Unit::Pound, Unit::Kilogram) => Some(value * 0.45359237),
        (Unit::Kilogram, Unit::Pound) => Some(value / 0.45359237),

        (Unit::Calorie, Unit::Joule) => Some(value * 4.184),
        (Unit::Joule, Unit::Calorie) => Some(value / 4.184),

        (Unit::PoundPerSquareInch, Unit::Kilopascal) => Some(value * 6.894757),
        (Unit::Kilopascal, Unit::PoundPerSquareInch) => Some(value / 6.894757),

        _ => None,
    }
}

#[cfg(test)] // currently only used in tests
fn delta_convert(delta: Float, from: Unit, to: Unit) -> Option<Float> {
    match (convert(0.0, from, to), convert(delta, from, to)) {
        (Some(base), Some(point)) => Some(point - base),
        _ => None,
    }
}

impl Quantity {
    pub fn unit_pairs(&self) -> Vec<(Unit, Unit)> {
        match self {
            Quantity::Temperature => vec![Unit::Celsius.pair_with(Unit::Fahrenheit)],
            Quantity::Length => vec![
                (Unit::Meter, Unit::Foot),
                (Unit::Kilometer, Unit::Mile),
                (Unit::Kilometer, Unit::NauticalMile),
                (Unit::NauticalMile, Unit::Mile),
            ],
            Quantity::Area => vec![(Unit::SquareFoot, Unit::SquareMeter), (Unit::Hectare, Unit::Acre)],
            Quantity::Volume => vec![(Unit::Millilitre, Unit::FluidOunce), (Unit::Gallon, Unit::Litre)],
            Quantity::Mass => vec![(Unit::Kilogram, Unit::Pound)],
            Quantity::Energy => vec![(Unit::Calorie, Unit::Joule)],
            Quantity::Pressure => vec![(Unit::Kilopascal, Unit::PoundPerSquareInch)],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Choice {
    pub unit: Unit,
    pub value: Float,
    pub equivalent: Float,
}

#[derive(Clone, Copy, Debug)]
pub enum ChoiceSelection {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Challenge {
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

    fn unit_pair(&self) -> (Unit, Unit) {
        self.left_choice.unit.pair_with(self.right_choice.unit)
    }

    fn generate(unit_pair: (Unit, Unit), level: Level) -> Self {
        assert_ne!(unit_pair.0, unit_pair.1);
        let (bigger, smaller) = if unit_pair.0 < unit_pair.1 {
            (unit_pair.1, unit_pair.0)
        } else {
            (unit_pair.0, unit_pair.1)
        };
        let delta = bigger.level_delta(level);
        let mid_point = {
            let min_allowed = bigger.min_value() + delta;
            let max_allowed = bigger.max_value() - delta;
            assert!(min_allowed < max_allowed);
            min_allowed + (max_allowed - min_allowed) * random::<Float>()
        };
        let (bigger_value, smaller_value) = {
            if random::<bool>() {
                (
                    (mid_point + delta).ceil(),
                    convert(mid_point - delta, bigger, smaller).unwrap().floor(),
                )
            } else {
                (
                    (mid_point - delta).floor(),
                    convert(mid_point + delta, bigger, smaller).unwrap().ceil(),
                )
            }
        };

        let bigger_choice = Choice {
            unit: bigger,
            value: bigger_value,
            equivalent: convert(bigger_value, bigger, smaller).unwrap(),
        };
        let smaller_choice = Choice {
            unit: smaller,
            value: smaller_value,
            equivalent: convert(smaller_value, smaller, bigger).unwrap(),
        };

        if random::<bool>() {
            Self {
                left_choice: bigger_choice,
                right_choice: smaller_choice,
            }
        } else {
            Self {
                left_choice: smaller_choice,
                right_choice: bigger_choice,
            }
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub in_progress: bool,
    level_per_unit_pair: HashMap<(Unit, Unit), Level>,
    pub challenge: Challenge,
}

impl Game {
    pub fn new_with_single_quantity(quantity: Quantity) -> Self {
        let mut level_per_unit_pair = HashMap::new();
        let unit_pairs = quantity.unit_pairs();
        for pair in &unit_pairs {
            level_per_unit_pair.insert(*pair, 0);
        }
        let mut rng = rand::thread_rng();
        Self {
            in_progress: true,
            level_per_unit_pair,
            challenge: Challenge::generate(unit_pairs.iter().choose(&mut rng).unwrap().clone(), 0),
        }
    }

    pub fn pick(&mut self, selection: ChoiceSelection) {
        if self.challenge.is_correct(selection) {
            self.level_per_unit_pair
                .entry(self.challenge.unit_pair())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            let mut rng = rand::thread_rng();
            let next_unit_pair = self.level_per_unit_pair.keys().choose(&mut rng).unwrap();
            let level = self.level_per_unit_pair.get(next_unit_pair).unwrap_or(&0).clone();
            self.challenge = Challenge::generate(*next_unit_pair, level);
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
        assert!(floats_close_enough(
            convert(0.0, Unit::Celsius, Unit::Celsius).unwrap(),
            0.0
        ));
        assert!(floats_close_enough(
            convert(0.0, Unit::Celsius, Unit::Fahrenheit).unwrap(),
            32.0
        ));
        assert!(floats_close_enough(
            convert(100.0, Unit::Celsius, Unit::Fahrenheit).unwrap(),
            212.0
        ));
        assert!(floats_close_enough(
            convert(0.0, Unit::Fahrenheit, Unit::Celsius).unwrap(),
            -17.777777
        ));
        assert!(floats_close_enough(
            convert(100.0, Unit::Fahrenheit, Unit::Celsius).unwrap(),
            37.777777
        ));

        assert_eq!(
            convert(0.0, Unit::Kilometer, Unit::NauticalMile),
            convert(0.0, Unit::NauticalMile, Unit::Kilometer)
        );
        assert!(floats_close_enough(
            convert(1000.0, Unit::NauticalMile, Unit::Kilometer).unwrap(),
            1852.0
        ));

        assert!(floats_close_enough(
            convert(8700.0, Unit::Joule, Unit::Calorie).unwrap(),
            2079.35
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
    fn units_order_follows_delta() {
        for a in ALL_UNITS {
            for b in ALL_UNITS {
                if let Some(delta) = delta_convert(1.0, *a, *b) {
                    if delta < 1.0 {
                        assert!(*a < *b, "delta = {:?} so {:?} < {:?}", delta, *a, *b);
                    } else if delta > 1.0 {
                        assert!(*a > *b, "delta = {:?} so {:?} > {:?}", delta, *a, *b);
                    } else {
                        assert_eq!(*a, *b, "delta = {:?} so {:?} == {:?}", delta, *a, *b);
                    }
                }
            }
        }
    }

    #[test]
    fn new_game_is_in_progress() {
        assert_eq!(Game::new_with_single_quantity(Quantity::Temperature).in_progress, true);
    }

    #[test]
    fn correct_pick_increases_level() {
        let mut level_per_unit_pair = HashMap::new();
        let unit_pair = Unit::Celsius.pair_with(Unit::Fahrenheit);
        level_per_unit_pair.insert(unit_pair, 3);
        let mut game = Game {
            in_progress: true,
            level_per_unit_pair,
            challenge: Challenge {
                left_choice: Choice {
                    unit: Unit::Celsius,
                    value: 30.0,
                    equivalent: 30.0,
                },
                right_choice: Choice {
                    unit: Unit::Fahrenheit,
                    value: 0.0,
                    equivalent: 0.0,
                },
            },
        };
        game.pick(ChoiceSelection::Left);
        assert_eq!(game.level_per_unit_pair.get(&unit_pair), Some(&4));
        assert_eq!(game.in_progress, true);
    }

    #[test]
    fn wrong_pick_stops_game() {
        let mut level_per_unit_pair = HashMap::new();
        let unit_pair = Unit::Celsius.pair_with(Unit::Fahrenheit);
        level_per_unit_pair.insert(unit_pair, 3);
        let mut game = Game {
            in_progress: true,
            level_per_unit_pair,
            challenge: Challenge {
                left_choice: Choice {
                    unit: Unit::Celsius,
                    value: 30.0,
                    equivalent: 30.0,
                },
                right_choice: Choice {
                    unit: Unit::Fahrenheit,
                    value: 0.0,
                    equivalent: 0.0,
                },
            },
        };
        game.pick(ChoiceSelection::Right);
        assert_eq!(game.level_per_unit_pair.get(&unit_pair), Some(&3));
        assert_eq!(game.in_progress, false);
    }
}
