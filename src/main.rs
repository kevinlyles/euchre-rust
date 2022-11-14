use game_state::GameState;
use log::LevelFilter;
use logger::Logger;
use player::Player;
use players::basic::BasicPlayer;
use position::Position;

mod bid_result;
mod bid_state;
mod card;
mod deck;
mod game_state;
mod hand;
mod hand_state;
mod logger;
mod player;
mod players;
mod position;
mod rank;
mod rank_with_bowers;
mod suit;
mod trick_state;

static LOGGER: Logger = Logger;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap_or_else(|_| println!("{}", "Logging initialization failed!"));
    let players: [Box<dyn Player>; 4] = [
        Box::new(BasicPlayer {
            position: Position::North,
        }),
        Box::new(BasicPlayer {
            position: Position::East,
        }),
        Box::new(BasicPlayer {
            position: Position::South,
        }),
        Box::new(BasicPlayer {
            position: Position::West,
        }),
    ];
    let mut game_state = GameState::create(players);
    loop {
        match game_state.step() {
            Some(result) => {
                println!("{}", result);
                break;
            }
            None => (),
        }
    }
}
