use std::{error::Error, path::PathBuf};

use clap::Parser;

use crate::models::{Card, Continent, Game, Player, Territory};

pub mod models;

/// The classic board game Risk
#[derive(Parser, Debug, Clone, Default, PartialEq, Eq)]
#[command(version, about, long_about = None)]
pub struct Args {}

/// Contains the configuration for running this program
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub player_count: PlayerCount,
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
pub fn run() -> Result<(), Box<dyn Error>> {
    // Initialize
    let territories_path = PathBuf::from("./assets/territories.json");
    let continents_path = PathBuf::from("./assets/continents.json");
    let cards_path = PathBuf::from("./assets/cards.json");

    let players: Vec<Player> = match build_players() {
        Err(e) => return Err(e),
        Ok(players) => players,
    };

    let territories = load_territories(territories_path);
    let continents = load_continents(continents_path);
    let cards = load_cards(cards_path);

    let mut game = Game::new(players, territories, continents, cards);

    // Game loop
    while game.is_running {
        game.take_turn();
        game.draw();
    }

    Ok(())
}

pub fn build_players() -> Result<Vec<Player>, Box<dyn Error>> {
    todo!()
}

pub fn load_territories(territories_path: PathBuf) -> Vec<Territory> {
    todo!()
}

pub fn load_continents(continents_path: PathBuf) -> Vec<Continent> {
    todo!()
}

pub fn load_cards(cards_path: PathBuf) -> Vec<Card> {
    todo!()
}
