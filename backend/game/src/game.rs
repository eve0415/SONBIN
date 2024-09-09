use std::collections::HashMap;
use std::time::SystemTime;
use serenity::all::UserId;
use board::board::Board;
use crate::config::{GameMode, GameSettings};

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
            id: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
            host,
            mode,
            settings,
            ..Default::default()
        }
    }

    pub fn join(&mut self, id: UserId) -> Result<Board, Error> {
        if let Some(max) = self.settings.max_player {
            if self.participants.len() == max {
                return Err(Error::MaxPlayers);
            }
        }

        if let Some(board) = self.participants.get(&id) {
            return Ok(**board);
        }

        if let Ok(board) = Board::new(id.get() + From::from(self.id),5) {
            self.participants.insert(id, board.clone());
            Ok(board)
        } else {
            Err(Error::BoardGenerationError )
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
