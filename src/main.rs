mod logic;

const KEY_TO_GAME: &[(&str, logic::Quantity)] = &[
    ("t", logic::Quantity::Temperature),
    ("l", logic::Quantity::Length),
    ("a", logic::Quantity::Area),
    ("v", logic::Quantity::Volume),
    ("m", logic::Quantity::Mass),
    ("e", logic::Quantity::Energy),
    ("p", logic::Quantity::Pressure),
];

fn main() {
    let mut quit = false;

    while !quit {
        let mut choice: Option<Vec<logic::Quantity>> = None;
        while !quit && choice.is_none() {
            for (key, quantity) in KEY_TO_GAME {
                println!("{} = {:?}", key, quantity);
            }
            println!("* = all");
            println!("q = quit");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            buf = buf.trim().to_string();
            for (key, quantity) in KEY_TO_GAME {
                if *key == buf {
                    choice = Some(vec![*quantity]);
                    break;
                }
            }
            match buf.as_str() {
                "*" => choice = Some(KEY_TO_GAME.iter().map(|(_, quantity)| *quantity).collect()),
                "q" => quit = true,
                _ => {}
            }
        }

        if quit {
            break;
        }

        // TODO: change to multiple quantities once implemented in logic
        let mut game = logic::Game::new_with_single_quantity(choice.unwrap().first().unwrap().clone());
        while !quit && game.in_progress {
            println!("{:?}", game);
            println!("1? 2? q?");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            match buf.trim() {
                "1" => game.pick(logic::ChoiceSelection::Left),
                "2" => game.pick(logic::ChoiceSelection::Right),
                "q" => quit = true,
                _ => println!("Invalid selection: {:?}", buf),
            }
        }
        println!("{:?}", game);
    }
}
