use std::error::Error;

use clap::Parser;

/// The classic board game Risk
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of players
    #[arg(short, long)]
    num_players: u32,
}

/// Contains the configuration for running this program
#[derive(Debug)]
pub struct Config {
    player_count: PlayerCount,
}

impl Config {
    pub fn build(args: Args) -> Result<Self, &'static str> {
        let player_count = match PlayerCount::try_from(args.num_players) {
            Ok(count) => count,
            Err(reason) => return Err(reason),
        };

        Ok(Config { player_count })
    }
}

/// Contains the number of players
#[derive(Debug)]
struct PlayerCount {
    value: u8,
}

impl TryFrom<u8> for PlayerCount {
    type Error = &'static str;

    fn try_from(count: u8) -> Result<Self, Self::Error> {
        if count < 2 || count > 6 {
            return Err("PlayerCount only accepts values between 2 and 6");
        }

        Ok(PlayerCount { value: count })
    }
}

impl TryFrom<u32> for PlayerCount {
    type Error = &'static str;

    fn try_from(count: u32) -> Result<Self, Self::Error> {
        if count < 2 || count > 6 {
            return Err("PlayerCount only accepts values between 2 and 6");
        }

        Ok(PlayerCount { value: count as u8 })
    }
}

impl Into<u8> for PlayerCount {
    fn into(self) -> u8 {
        self.value
    }
}

impl Into<u32> for PlayerCount {
    fn into(self) -> u32 {
        self.value as u32
    }
}

/// Runs this program with the given configuration
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("num-players = {}", Into::<u32>::into(config.player_count));

    Ok(())
}
