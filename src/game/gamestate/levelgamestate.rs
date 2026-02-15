use macroquad::prelude::*;
use std::rc::Rc;

use crate::game::soundstore::SoundStore;

mod bullet;
mod enemy;
mod player;
mod world;

use self::world::World;

use super::{finalgamestate::FinalGameState, pausegamestate::PauseGameState, GameState, GameStateResult};

pub struct LevelGameState {
    world: World,
    sound_store: Rc<SoundStore>
}

impl LevelGameState {
    pub fn new(sound_store: Rc<SoundStore>) -> Self {
        let result = Self {
            world: World::new(),
            sound_store
        };
        result.sound_store.play_background_music();
        return result;
    }
}

impl GameState for LevelGameState {
    fn update(&mut self) -> Option<GameStateResult>  {
        if is_key_pressed(KeyCode::P) {
            return Some(super::GameStateResult {
                pop: false,
                game_state: Some(Box::new(PauseGameState{}))
            })            
        }

        if is_key_pressed(KeyCode::Escape) {
            return Some(super::GameStateResult {
                pop: true,
                game_state: None
            })            
        }

        if self.world.has_no_enemies() {
            self.sound_store.play_win_sound();
            self.sound_store.stop_background_music();
            return Some(super::GameStateResult {
                pop: true,
                game_state: Some(Box::new(FinalGameState::new(true, self.sound_store.clone())))
            })
        }

        if !self.world.is_player_alive() {
            self.sound_store.play_dead_sound();
            self.sound_store.stop_background_music();
            return Some(super::GameStateResult {
                pop: true,
                game_state: Some(Box::new(FinalGameState::new(false, self.sound_store.clone())))
            })
        }

        self.world.update(get_frame_time(), self.sound_store.as_ref());

        None
    }
    
    fn draw(&self) {
        clear_background(Color::from_rgba(0, 10, 35, 255));
        self.world.draw();
    }
}

