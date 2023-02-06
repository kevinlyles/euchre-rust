use clap::{command, ArgGroup, Args, Parser, Subcommand};

use crate::{card::CardBeforeBidding, position::Position, suit::Suit};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(group = ArgGroup::new("simulate-hand").multiple(false))]
pub(crate) struct EuchreArgs {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    PlayGame,
    SimulateHand(SimulateHandArgs),
}

#[derive(Args)]
pub(crate) struct SimulateHandArgs {
    #[arg(long, required = true, value_name = "trump candidate")]
    pub(crate) trump_candidate: CardBeforeBidding,

    #[arg(long, required = true, value_name = "position")]
    pub(crate) dealer: Position,

    #[arg(
        long,
        action,
        required_unless_present("call_suit"),
        conflicts_with("call_suit")
    )]
    pub(crate) order_up: bool,

    #[arg(long, required_unless_present("order_up"), value_name = "trump suit")]
    pub(crate) call_suit: Option<Suit>,

    #[arg(long, action)]
    pub(crate) go_alone: bool,

    #[arg(long, action)]
    pub(crate) ignore_other_bids: bool,

    #[arg(long, required = true, num_args = 5, value_name = "card")]
    pub(crate) hand: Vec<CardBeforeBidding>,
}
