mod logic;

fn main() {
    let mut game = logic::Game::new(logic::GameType::Temperature);
    let mut quit = false;
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
}
