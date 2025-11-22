use std::error::Error;

use clap::Parser;

pub mod models;

/// The classic board game Risk
#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of players
    #[arg(short, long)]
    num_players: u32,
}

/// Contains the configuration for running this program
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub player_count: PlayerCount,
}

impl Config {
    /// Builds a Config from command line arguments
    pub fn build(args: Args) -> Result<Self, &'static str> {
        let player_count = match PlayerCount::try_from(args.num_players) {
            Ok(count) => count,
            Err(reason) => return Err(reason),
        };

        Ok(Config { player_count })
    }
}

/// Contains the number of players
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct PlayerCount {
    pub value: u8,
}

impl TryFrom<u32> for PlayerCount {
    type Error = &'static str;

    /// Tries to convert a u32 to a PlayerCount
    fn try_from(count: u32) -> Result<Self, Self::Error> {
        if count < 2 || count > 6 {
            return Err("PlayerCount only accepts values between 2 and 6");
        }

        Ok(PlayerCount { value: count as u8 })
    }
}

impl Into<u32> for PlayerCount {
    /// Tries to convert a PlayerCount to a u32
    fn into(self) -> u32 {
        self.value as u32
    }
}

/// Runs a game of Risk with the given configuration
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    todo!("Implement the game logic for a game of Risk")
}
