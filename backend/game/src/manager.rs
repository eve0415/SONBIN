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
