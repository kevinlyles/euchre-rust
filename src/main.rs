#![forbid(non_ascii_idents)]
#![warn(let_underscore)]
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

use args::{Commands, EuchreArgs, SimulateHandArgs};
use bid_result::BidResultCalled;
use clap::Parser;
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
use std::collections::HashMap;

mod args;
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
    let args = EuchreArgs::parse();
    match args.command {
        Commands::PlayGame => simulate_full_game(),
        Commands::SimulateHand(args) => simulate_hand(args),
    }
}

fn simulate_hand(args: SimulateHandArgs) {
    let (bidder, expected_bid_result) = get_bidding_info(&args);
    let hand_states = HandState::create_with_scenario(
        args.dealer,
        args.trump_candidate,
        HandBeforeBidding { cards: args.hand },
    );
    let (total_count, result_counts) = hand_states
        .map_with(
            (bidder, expected_bid_result),
            |(bidder, expected_bid_result), mut hand_state| {
                run_permutation(
                    bidder,
                    expected_bid_result,
                    &mut hand_state,
                    args.ignore_other_bids,
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
}

fn get_bidding_info(args: &SimulateHandArgs) -> (PreprogrammedBidder, BidResultCalled) {
    //TODO: handle defending alone somehow
    let (bidder, bid_result) = if args.order_up {
        if args.go_alone {
            (
                PreprogrammedBidder::orders_up_alone(),
                BidResultCalled::CalledAlone {
                    trump: args.trump_candidate.suit,
                    caller: Position::South,
                },
            )
        } else {
            (
                PreprogrammedBidder::orders_up(),
                BidResultCalled::Called {
                    trump: args.trump_candidate.suit,
                    caller: Position::South,
                },
            )
        }
    } else if let Some(trump) = args.call_suit {
        if args.go_alone {
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
    (bidder, bid_result)
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
        if let Some((winner, score)) = hand_state.step(&mut players) {
            return if winner == Position::South || winner == Position::South.partner() {
                HandResult::ExpectedBidResult { score: score as i8 }
            } else {
                HandResult::ExpectedBidResult {
                    score: -(score as i8),
                }
            };
        }
    }
}

fn add_to_results(
    result_counts: &mut HashMap<HandResult, u64>,
    hand_result: HandResult,
    count: u64,
) {
    match result_counts.get_mut(&hand_result) {
        Some(result_count) => {
            *result_count += count;
        }
        None => {
            if let Some(old_value) = result_counts.insert(hand_result, count) {
                panic!(
                    "Got an old value after get_mut returned None: {}",
                    old_value
                )
            }
        }
    };
}

fn tally_results(result_counts: HashMap<HandResult, u64>, total_count: u64) {
    let mut results: Vec<&HandResult> = result_counts.keys().collect();
    results.sort();
    let mut expected_value: i64 = 0;
    let mut total_bid_count: u64 = 0;
    println!("Results:");
    for result in results {
        let count = result_counts.get(result).unwrap();
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

fn print_score_line(description: &str, count: &u64, total_count: &u64) {
    println!(
        "{} {} times ({:.2}%)",
        description,
        count.to_formatted_string(&Locale::en),
        (*count as f64) / (*total_count as f64) * 100f64
    )
}

fn simulate_full_game() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap_or_else(|_| println!("Logging initialization failed!"));
    let players = [
        AdvancedPlayer::create(Position::North),
        AdvancedPlayer::create(Position::East),
        AdvancedPlayer::create(Position::South),
        AdvancedPlayer::create(Position::West),
    ];
    let mut game_state = GameState::create(players);
    loop {
        if let Some(result) = game_state.step() {
            println!("{}", result);
            break;
        }
    }
}
