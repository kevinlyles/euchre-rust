use bid_result::BidResultCalled;
use card::Card;
use game_state::GameState;
use hand::Hand;
use hand_state::HandState;
use log::LevelFilter;
use logger::Logger;
use num_format::{Locale, ToFormattedString};
use players::basic::BasicPlayer;
use position::Position;
use rayon::prelude::ParallelIterator;
use std::{collections::HashMap, env};
use suit::Suit;

mod bid_result;
mod bid_state;
mod card;
mod deck;
mod game_state;
mod hand;
mod hand_state;
mod hands_iterator;
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
    let args: Vec<String> = env::args().collect();
    let players = [
        BasicPlayer {
            position: Position::North,
        },
        BasicPlayer {
            position: Position::East,
        },
        BasicPlayer {
            position: Position::South,
        },
        BasicPlayer {
            position: Position::West,
        },
    ];
    if args.contains(&"--simulate-hand".to_owned()) {
        let (hand, trump_candidate, dealer, bid_result) = process_args(args);
        let hand_states =
            HandState::create_with_scenario(hand, trump_candidate, &bid_result.trump());
        let (total_count, result_counts) = hand_states
            .map_with(
                (players.clone(), bid_result.clone()),
                |(players, bid_result), hands| {
                    HandState::create_hand_state(
                        players,
                        dealer,
                        hands,
                        trump_candidate,
                        bid_result,
                    )
                },
            )
            .map_with(players.clone(), |players, mut hand_state| loop {
                match hand_state.step(players) {
                    Some((winner, score)) => {
                        return if winner == Position::South || winner == Position::South.partner() {
                            score as i8
                        } else {
                            -(score as i8)
                        }
                    }
                    None => (),
                }
            })
            .fold(
                || (0, HashMap::<i8, u64>::new()),
                |(count, result_counts), score| {
                    let mut new_result_counts = result_counts.clone();
                    match new_result_counts.get_mut(&score) {
                        Some(score_count) => {
                            *score_count += 1;
                        }
                        None => {
                            new_result_counts.insert(score, 1);
                        }
                    };
                    (count + 1, new_result_counts)
                },
            )
            .reduce(
                || (0, HashMap::<i8, u64>::new()),
                |(count_1, result_counts_1), (count_2, result_counts_2)| {
                    let mut new_result_counts = result_counts_1.clone();
                    for (score, count) in result_counts_2.iter() {
                        match new_result_counts.get_mut(score) {
                            Some(score_count) => {
                                *score_count += count;
                            }
                            None => {
                                new_result_counts.insert(*score, *count);
                            }
                        }
                    }
                    (count_1 + count_2, new_result_counts)
                },
            );
        let mut scores: Vec<&i8> = result_counts.keys().collect();
        scores.sort();
        let mut expected_value: i64 = 0;
        println!("Results:");
        for &score in scores {
            let count = result_counts.get(&score).unwrap();
            match score {
                -4 => {
                    print_score_line("Opponent successfully defended alone", count, &total_count);
                }
                -2 => {
                    print_score_line("Opponent euchred you", count, &total_count);
                }
                1 => {
                    print_score_line("You made it", count, &total_count);
                }
                2 => {
                    print_score_line("You took all 5 tricks", count, &total_count);
                }
                4 => {
                    print_score_line("You made it alone", count, &total_count);
                }
                score => {
                    print_score_line(
                        format!("Unexpected score {}", score).as_str(),
                        count,
                        &total_count,
                    );
                }
            }
            expected_value += (score as i64) * (*count as i64);
        }
        println!(
            "Expected value: {}",
            (expected_value as f64) / (total_count as f64)
        )
    } else {
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
}

fn process_args(args: Vec<String>) -> (Hand, Card, Position, BidResultCalled) {
    let simulate_args: Vec<String> = args
        .into_iter()
        .skip_while(|arg| arg != "--simulate-hand")
        .skip(1)
        .collect();
    let mut hand = Hand { cards: Vec::new() };
    let mut trump_candidate = None;
    let mut dealer = None;
    let mut order_up = false;
    let mut call_suit = None;
    let mut go_alone = false;
    let mut i = 0;
    while i < simulate_args.len() {
        match simulate_args[i].as_str() {
            "--trump-candidate"
                if matches!(trump_candidate, None) && i + 1 < simulate_args.len() =>
            {
                match Card::try_create(simulate_args[i + 1].as_str()) {
                    Some(card) => trump_candidate = Some(card),
                    None => panic!("Invalid card: {}", simulate_args[i + 1]),
                };
                i += 1;
            }
            "--dealer" if matches!(dealer, None) && i + 1 < simulate_args.len() => {
                dealer = Some(match simulate_args[i + 1].as_str() {
                    "N" => Position::North,
                    "E" => Position::East,
                    "S" => Position::South,
                    "W" => Position::West,
                    other => panic!("Invalid dealer: {}", other),
                });
                i += 1;
            }
            "--order-up" if order_up == false && matches!(call_suit, None) => order_up = true,
            "--call-suit"
                if order_up == false
                    && matches!(call_suit, None)
                    && i + 1 < simulate_args.len() =>
            {
                match Suit::try_create(simulate_args[i + 1].as_str()) {
                    Some(suit) => call_suit = Some(suit),
                    None => panic!("Invalid suit: {}", simulate_args[i + 1]),
                };
                i += 1;
            }
            "--go-alone" if go_alone == false => go_alone = true,
            card => match Card::try_create(card) {
                Some(card) => hand.cards.push(card),
                None => panic!("Invalid card: {}", card),
            },
        }
        i += 1;
    }
    if let None = dealer {
        panic!("No dealer specified (use --dealer {{N|E|S|W}})");
    }
    let dealer = dealer.unwrap();
    if let None = trump_candidate {
        panic!("No trump candidate specified (use --trump-candidate {{A|K|Q|J|T|N}}{{C|D|H|S}})");
    }
    let trump_candidate = trump_candidate.unwrap();
    if hand.cards.len() != 5 {
        panic!(
                "Incorrect number of cards specified: {} (expected 5). Card format: {{A|K|Q|J|T|N}}{{C|D|H|S}}",
                hand.cards.len()
            );
    }
    let bid_result = if order_up {
        //TODO: handle defending alone?
        if go_alone {
            BidResultCalled::CalledAlone {
                trump: trump_candidate.suit,
                caller: Position::South,
            }
        } else {
            BidResultCalled::Called {
                trump: trump_candidate.suit,
                caller: Position::South,
            }
        }
    } else if let Some(suit) = call_suit {
        if go_alone {
            BidResultCalled::Called {
                trump: suit,
                caller: Position::South,
            }
        } else {
            BidResultCalled::CalledAlone {
                trump: suit,
                caller: Position::South,
            }
        }
    } else {
        panic!("You must either order the trump candidate up (--order-up) or call a trump suit (--call-suit {{C|D|H|S}}");
    };
    (hand, trump_candidate, dealer, bid_result)
}

fn print_score_line(description: &str, count: &u64, total_count: &u64) -> () {
    println!(
        "{} {} times ({:.2}%)",
        description,
        count.to_formatted_string(&Locale::en),
        (*count as f64) / (*total_count as f64) * 100f64
    )
}
