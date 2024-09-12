use crate::config::{GameMode, GameSettings};
use crate::errors::Error;
use board::board::Board;
use serde::Serialize;
use serenity::all::UserId;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    pub(crate) id: u32,
    pub(crate) host: UserId,
    pub(crate) mode: GameMode,
    pub(crate) settings: GameSettings,
    participants: HashMap<UserId, Board>,
}

impl Game {
    pub(crate) fn new(host: UserId, mode: GameMode, settings: GameSettings) -> (u32, Self) {
        let id = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();

        (
            id,
            Game {
                id,
                host,
                mode,
                settings,
                participants: HashMap::new(),
            },
        )
    }

    pub(crate) fn join(&mut self, id: UserId) -> Result<Board, Error> {
        if let Some(max) = self.settings.max_player {
            if self.participants.len() == max {
                return Err(Error::MaxPlayers);
            }
        }

        if let Some(board) = self.participants.get(&id) {
            return Ok(board.clone());
        }

        if let Ok(board) = Board::new(id.get() + u64::from(self.id), 5) {
            self.participants.insert(id, board.clone());
            Ok(board)
        } else {
            Err(Error::BoardGenerationError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_game() {
        let user = UserId::default();
        let (id, game) = Game::new(user, GameMode::NORMAL, GameSettings::default());

        assert_eq!(id, game.id);
        assert_eq!(user, game.host);
    }

    #[test]
    fn it_can_join_game() {
        let user = UserId::default();

        let (_, mut game) = Game::new(user, GameMode::NORMAL, GameSettings::default());
        let board = game.join(user).unwrap();

        assert_eq!(user.get() + u64::from(game.id), board.id);
        assert_eq!(board, game.join(user).unwrap());
    }
}
