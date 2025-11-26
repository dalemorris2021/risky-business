use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

use rand::{rngs::ThreadRng, seq::SliceRandom};

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
    pub id: u32,
    pub name: String,
    pub player_armies: i32,
    pub player_cards: Vec<Card>,
}

impl Player {
    pub fn take_turn(game: &mut Game) {
        todo!()
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
    pub rng: ThreadRng,
    pub is_running: bool,
    pub players: Vec<Player>,
    pub turn: Box<dyn Iterator<Item = u32>>,
    pub territories: Vec<Territory>,
    pub continents: Vec<Continent>,
    pub deck: Vec<Card>,
    pub available_actions: Vec<Action>,
    pub taken_actions: Vec<Action>,
}

impl Game {
    /// Creates a new Game with the given data
    pub fn new(
        players: Vec<Player>,
        territories: Vec<Territory>,
        continents: Vec<Continent>,
        cards: Vec<Card>,
    ) -> Self {
        let mut rng = rand::rng();
        let is_running = true;

        let mut players = players;
        players.shuffle(&mut rng);
        let players = players;

        let mut turns: Vec<u32> = Vec::new();
        for player in &players {
            turns.push(player.id);
        }
        let turn = Box::new(turns.into_iter().cycle());

        let mut deck = cards;
        deck.shuffle(&mut rng);

        let available_actions: Vec<Action> = Vec::new();
        let taken_actions: Vec<Action> = Vec::new();

        Game {
            rng,
            is_running,
            players,
            turn,
            territories,
            continents,
            deck,
            available_actions,
            taken_actions,
        }
    }

    pub fn take_turn(&mut self) {
        todo!()
    }

    pub fn draw(&self) {
        todo!()
    }
}
