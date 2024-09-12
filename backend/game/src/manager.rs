use crate::config::{GameMode, GameSettings};
use crate::errors::Error;
use crate::game::Game;
use board::board::Board;
use serenity::all::UserId;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct GameManager {
    games: HashMap<u32, Game>,
}

impl GameManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_game(
        &mut self,
        host: UserId,
        mode: GameMode,
        settings: GameSettings,
    ) -> Result<Game, Error> {
        // Host cannot create a game when they have ongoing games
        for game in self.games.values() {
            if game.host == host {
                return Err(Error::OngoingGame {
                    host,
                    game_id: game.id,
                });
            }
        }

        let (id, game) = Game::new(host, mode, settings);

        self.games.insert(id, game.clone());

        Ok(game)
    }

    pub fn join_game(&mut self, game_id: &u32, user_id: UserId) -> Result<Board, Error> {
        if let Some(game) = self.games.get_mut(game_id) {
            Ok(game.join(user_id)?)
        } else {
            Err(Error::NotFound { game_id: *game_id })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_game() {
        let mut manager = GameManager::default();
        let game = manager
            .create_game(UserId::default(), GameMode::NORMAL, GameSettings::default())
            .unwrap();

        assert_eq!(manager.games.get(&game.id).unwrap().id, game.id);
    }

    #[test]
    fn it_cannot_create_game() {
        let mut manager = GameManager::default();
        let user = UserId::default();

        let game = manager
            .create_game(user, GameMode::NORMAL, GameSettings::default())
            .unwrap();
        let err = manager
            .create_game(user, GameMode::NORMAL, GameSettings::default())
            .unwrap_err();

        assert_eq!(
            Error::OngoingGame {
                game_id: game.id,
                host: user
            },
            err
        );
    }

    #[test]
    fn it_can_join_game() {
        let mut manager = GameManager::default();
        let user = UserId::default();

        let game = manager
            .create_game(UserId::default(), GameMode::NORMAL, GameSettings::default())
            .unwrap();

        let board = manager.join_game(&game.id, user).unwrap();

        assert_eq!(user.get() + u64::from(game.id), board.id);
        assert_eq!(board, manager.join_game(&game.id, user).unwrap());
    }

    #[test]
    fn it_cannot_join_unknown_game() {
        let mut manager = GameManager::default();

        let err = manager.join_game(&1, UserId::default()).unwrap_err();

        assert_eq!(Error::NotFound { game_id: 1 }, err);
    }
}
