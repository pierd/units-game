use rand::random;

#[derive(Debug)]
pub enum Unit {
    Celsius,
    Fahrenheit,
}

type Float = f32;
type Level = usize;

fn celsius_to_fahrenheit(celsius: Float) -> Float {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: Float) -> Float {
    (fahrenheit - 32.0) / 1.8
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameType {
    Temperatures,
}

#[derive(Debug)]
pub struct Challenge {
    left_choice: Choice,
    right_choice: Choice,
}

impl Challenge {
    fn is_correct(&self, selection: ChoiceSelection) -> bool {
        match selection {
            ChoiceSelection::Left => self.left_choice.value > self.right_choice.equivalent,
            ChoiceSelection::Right => self.right_choice.value > self.left_choice.equivalent,
        }
    }
}

#[derive(Debug)]
pub struct Choice {
    unit: Unit,
    value: Float,
    equivalent: Float,
}

#[derive(Debug)]
pub enum ChoiceSelection {
    Left,
    Right,
}

impl GameType {
    pub fn generate_challenge(&self, level: Level) -> Challenge {
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

        let left_choice = Choice {
            unit: Unit::Celsius,
            value: c_temperature,
            equivalent: celsius_to_fahrenheit(c_temperature)
        };
        let right_choice = Choice {
            unit: Unit::Fahrenheit,
            value: f_temperature,
            equivalent: fahrenheit_to_celsius(f_temperature)
        };
        if random::<bool>() {
            Challenge {
                left_choice,
                right_choice,
            }
        } else {
            Challenge {
                left_choice: right_choice,
                right_choice: left_choice,
            }
        }
    }
}

#[derive(Debug)]
pub struct UnitsGame {
    pub game_type: GameType,
    pub in_progress: bool,
    pub level: Level,
    pub challenge: Challenge,
}

impl UnitsGame {
    pub fn new(game_type: GameType) -> Self {
        Self {
            game_type,
            in_progress: true,
            level: 0,
            challenge: game_type.generate_challenge(0),
        }
    }

    pub fn pick(&mut self, selection: ChoiceSelection) {
        if self.challenge.is_correct(selection) {
            self.level += 1;
            self.challenge = self.game_type.generate_challenge(self.level);
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
    fn temperature_conversations_work() {
        assert_eq!(celsius_to_fahrenheit(-40.0), fahrenheit_to_celsius(-40.0));
        assert!(floats_close_enough(celsius_to_fahrenheit(0.0), 32.0));
        assert!(floats_close_enough(celsius_to_fahrenheit(100.0), 212.0));
        assert!(floats_close_enough(fahrenheit_to_celsius(0.0), -17.777777));
        assert!(floats_close_enough(fahrenheit_to_celsius(100.0), 37.777777));
    }

    #[test]
    fn new_game_is_in_progress() {
        assert_eq!(UnitsGame::new(GameType::Temperatures).in_progress, true);
    }

    #[test]
    fn correct_pick_increases_level() {
        let mut game = UnitsGame {
            game_type: GameType::Temperatures,
            in_progress: true,
            level: 3,
            challenge: Challenge {
                left_choice: Choice { unit: Unit::Celsius, value: 30.0, equivalent: 30.0 },
                right_choice: Choice { unit: Unit::Celsius, value: 10.0, equivalent: 10.0 },
            },
        };
        game.pick(ChoiceSelection::Left);
        assert_eq!(game.level, 4);
        assert_eq!(game.in_progress, true);
    }

    #[test]
    fn wrong_pick_stops_game() {
        let mut game = UnitsGame {
            game_type: GameType::Temperatures,
            in_progress: true,
            level: 3,
            challenge: Challenge {
                left_choice: Choice { unit: Unit::Celsius, value: 30.0, equivalent: 30.0 },
                right_choice: Choice { unit: Unit::Celsius, value: 10.0, equivalent: 10.0 },
            },
        };
        game.pick(ChoiceSelection::Right);
        assert_eq!(game.level, 3);
        assert_eq!(game.in_progress, false);
    }
}
