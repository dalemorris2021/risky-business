use std::{collections::HashMap, hash::Hash, hash::Hasher, rc::Rc};

use rand::rngs::ThreadRng;

/// The required behavior of an agent that can participate in an instance of a game
pub trait Playable {
    fn take_turn(game: &mut Game);
}

/// A game action taken by a player
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Deploy,
    Attack,
    Fortify,
}

/// The type of a Risk game Card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardType {
    Infantry,
    Cavalry,
    Artillery,
    Wild,
}

/// A Risk game card
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: u32,
    pub card_type: CardType,
    pub name: Option<String>,
}

impl Hash for Card {
    /// Hashes a card based on its id
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// A player of a game of Risk
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub name: String,
}

impl Playable for Player {
    fn take_turn(game: &mut Game) {
        todo!("Implement logic for player taking a turn");
    }
}

/// A Risk game territory
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Territory {
    pub name: String,
    pub neighbors: Vec<Rc<Territory>>,
    pub occupying_player: Option<Player>,
    pub num_armies: i32,
}

/// A Risk game continent
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Continent {
    pub name: String,
    pub army_bonus: u32,
    pub territories: Vec<Rc<Territory>>,
}

/// A game of Risk
pub struct Game {
    pub deck: Vec<Card>,
    pub players: HashMap<u32, Player>,
    pub player_armies: HashMap<u32, i32>,
    pub player_cards: HashMap<u32, Vec<Card>>,
    pub turn: Box<dyn Iterator<Item = Player>>,
    pub territories: Vec<Territory>,
    pub continents: Vec<Continent>,
    pub available_actions: Vec<Action>,
    pub taken_actions: Vec<Action>,
    pub rand: ThreadRng,
}
