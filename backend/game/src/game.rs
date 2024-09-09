use crate::config::{GameMode, GameSettings};
use board::board::Board;
use serenity::all::UserId;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Game {
    id: u32,
    pub host: UserId,
    pub mode: GameMode,
    pub settings: GameSettings,
    participants: HashMap<UserId, Board>,
}

impl Game {
    pub fn new(host: UserId, mode: GameMode, settings: GameSettings) -> Self {
        Game {
            id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos(),
            host,
            mode,
            settings,
            participants: HashMap::new(),
        }
    }

    pub fn join(&mut self, id: UserId) -> Result<Board, Error> {
        if let Some(max) = self.settings.max_player {
            if self.participants.len() == max {
                return Err(Error::MaxPlayers);
            }
        }

        if let Some(board) = self.participants.get(&id) {
            return Ok(board.clone());
        }

        if let Ok(board) = Board::new(id.get() + u64::from(self.id),5) {
            self.participants.insert(id, board.clone());
            Ok(board)
        } else {
            Err(Error::BoardGenerationError)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Max players reached")]
    MaxPlayers,

    #[error("Cannot generate board")]
    BoardGenerationError,
}

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
