#![forbid(non_ascii_idents)]
#![warn(clippy::let_underscore)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused)]
#![warn(unused_crate_dependencies)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(unused_tuple_struct_fields)]
#![warn(variant_size_differences)]

use bid_result::BidResultCalled;
use card::CardBeforeBidding;
use game_state::GameState;
use hand::HandBeforeBidding;
use hand_state::HandState;
use log::LevelFilter;
use logger::Logger;
use num_format::{Locale, ToFormattedString};
use players::{
    advanced::AdvancedPlayer, preprogrammed_bidder::PreprogrammedBidder, wrapper::Wrapper,
};
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

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum HandResult {
    DifferentBidResult,
    ExpectedBidResult { score: i8 },
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--simulate-hand".to_owned()) {
        let (hand, trump_candidate, dealer, bidder, expected_bid_result, ignore_other_bids) =
            process_args(args);
        let hand_states = HandState::create_with_scenario(dealer, trump_candidate, hand);
        let (total_count, result_counts) = hand_states
            .map_with(
                (bidder, expected_bid_result),
                |(bidder, expected_bid_result), mut hand_state| {
                    run_permutation(
                        bidder,
                        expected_bid_result,
                        &mut hand_state,
                        ignore_other_bids,
                    )
                },
            )
            .fold(
                || (0, HashMap::<HandResult, u64>::new()),
                |(count, mut result_counts), hand_result| {
                    add_to_results(&mut result_counts, hand_result, 1);
                    (count + 1, result_counts)
                },
            )
            .reduce(
                || (0, HashMap::<HandResult, u64>::new()),
                |(count_1, mut result_counts_1), (count_2, result_counts_2)| {
                    for (result, count) in result_counts_2.into_iter() {
                        add_to_results(&mut result_counts_1, result, count);
                    }
                    (count_1 + count_2, result_counts_1)
                },
            );
        tally_results(result_counts, total_count);
    } else {
        simulate_full_game();
    }
}

fn process_args(
    args: Vec<String>,
) -> (
    HandBeforeBidding,
    CardBeforeBidding,
    Position,
    PreprogrammedBidder,
    BidResultCalled,
    bool,
) {
    let simulate_args: Vec<String> = args
        .into_iter()
        .skip_while(|arg| arg != "--simulate-hand")
        .skip(1)
        .collect();
    let mut hand = HandBeforeBidding { cards: Vec::new() };
    let mut trump_candidate = None;
    let mut dealer = None;
    let mut order_up = false;
    let mut call_suit = None;
    let mut go_alone = false;
    let mut ignore_other_bids = false;
    let mut i = 0;
    while i < simulate_args.len() {
        match simulate_args[i].as_str() {
            "--trump-candidate"
                if matches!(trump_candidate, None) && i + 1 < simulate_args.len() =>
            {
                match CardBeforeBidding::try_create(simulate_args[i + 1].as_str()) {
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
            "--ignore-other-bids" => ignore_other_bids = true,
            card => match CardBeforeBidding::try_create(card) {
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
    //TODO: handle defending alone by running the bidding process as well
    let (bidder, bid_result) = if order_up {
        if go_alone {
            (
                PreprogrammedBidder::orders_up_alone(),
                BidResultCalled::CalledAlone {
                    trump: trump_candidate.suit,
                    caller: Position::South,
                },
            )
        } else {
            (
                PreprogrammedBidder::orders_up(),
                BidResultCalled::Called {
                    trump: trump_candidate.suit,
                    caller: Position::South,
                },
            )
        }
    } else if let Some(trump) = call_suit {
        if go_alone {
            (
                PreprogrammedBidder::calls_alone(trump),
                BidResultCalled::CalledAlone {
                    trump,
                    caller: Position::South,
                },
            )
        } else {
            (
                PreprogrammedBidder::calls(trump),
                BidResultCalled::Called {
                    trump,
                    caller: Position::South,
                },
            )
        }
    } else {
        panic!("You must either order the trump candidate up (--order-up) or call a trump suit (--call-suit {{C|D|H|S}}");
    };
    (
        hand,
        trump_candidate,
        dealer,
        bidder,
        bid_result,
        ignore_other_bids,
    )
}

fn run_permutation(
    bidder: &mut PreprogrammedBidder,
    expected_bid_result: &BidResultCalled,
    hand_state: &mut HandState,
    ignore_other_bids: bool,
) -> HandResult {
    let mut players = [
        if ignore_other_bids {
            Wrapper::create_separate_bidder(
                Box::new(PreprogrammedBidder::does_nothing()),
                Box::new(AdvancedPlayer::create(Position::North)),
            )
        } else {
            Wrapper::create_single_player(Box::new(AdvancedPlayer::create(Position::North)))
        },
        if ignore_other_bids {
            Wrapper::create_separate_bidder(
                Box::new(PreprogrammedBidder::does_nothing()),
                Box::new(AdvancedPlayer::create(Position::East)),
            )
        } else {
            Wrapper::create_single_player(Box::new(AdvancedPlayer::create(Position::East)))
        },
        Wrapper::create_separate_bidder(
            Box::new(bidder.clone()),
            Box::new(AdvancedPlayer::create(Position::South)),
        ),
        if ignore_other_bids {
            Wrapper::create_separate_bidder(
                Box::new(PreprogrammedBidder::does_nothing()),
                Box::new(AdvancedPlayer::create(Position::West)),
            )
        } else {
            Wrapper::create_single_player(Box::new(AdvancedPlayer::create(Position::West)))
        },
    ];
    match hand_state.finish_bidding(&mut players) {
        Some(bid_result) if bid_result == *expected_bid_result => (),
        _ => return HandResult::DifferentBidResult,
    };
    loop {
        match hand_state.step(&mut players) {
            Some((winner, score)) => {
                return if winner == Position::South || winner == Position::South.partner() {
                    HandResult::ExpectedBidResult { score: score as i8 }
                } else {
                    HandResult::ExpectedBidResult {
                        score: -(score as i8),
                    }
                }
            }
            None => (),
        }
    }
}

fn add_to_results(
    result_counts: &mut HashMap<HandResult, u64>,
    hand_result: HandResult,
    count: u64,
) -> () {
    match result_counts.get_mut(&hand_result) {
        Some(result_count) => {
            *result_count += count;
        }
        None => match result_counts.insert(hand_result, count) {
            Some(old_value) => panic!(
                "Got an old value after get_mut returned None: {}",
                old_value
            ),
            None => (),
        },
    };
}

fn tally_results(result_counts: HashMap<HandResult, u64>, total_count: u64) -> () {
    let mut results: Vec<&HandResult> = result_counts.keys().collect();
    results.sort();
    let mut expected_value: i64 = 0;
    let mut total_bid_count: u64 = 0;
    println!("Results:");
    for result in results {
        let count = result_counts.get(&result).unwrap();
        match result {
            HandResult::DifferentBidResult => {
                print_score_line("You didn't get to bid", count, &total_count);
            }
            HandResult::ExpectedBidResult { score } => {
                total_bid_count += count;
                match score {
                    -4 => {
                        print_score_line(
                            "Opponent successfully defended alone",
                            count,
                            &total_count,
                        );
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
                };
                expected_value += (*score as i64) * (*count as i64);
            }
        }
    }
    println!(
        "Expected value when you're able to bid: {:.2}",
        (expected_value as f64) / (total_bid_count as f64)
    )
}

fn print_score_line(description: &str, count: &u64, total_count: &u64) -> () {
    println!(
        "{} {} times ({:.2}%)",
        description,
        count.to_formatted_string(&Locale::en),
        (*count as f64) / (*total_count as f64) * 100f64
    )
}

fn simulate_full_game() -> () {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap_or_else(|_| println!("{}", "Logging initialization failed!"));
    let players = [
        AdvancedPlayer::create(Position::North),
        AdvancedPlayer::create(Position::East),
        AdvancedPlayer::create(Position::South),
        AdvancedPlayer::create(Position::West),
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
