use macroquad::prelude::*;
use std::rc::Rc;

use crate::game::soundstore::SoundStore;

use super::{levelgamestate::LevelGameState, GameState, GameStateResult};

pub struct FinalGameState {
    won: bool,
    sound_store: Rc<SoundStore>
}

impl FinalGameState {
    pub fn new(won: bool, sound_store: Rc<SoundStore>) -> Self {
        Self {
            won,
            sound_store
        }
    }
}

impl GameState for FinalGameState {
    fn update(&mut self) -> Option<GameStateResult>  {
        if is_key_pressed(KeyCode::Escape) {
            return Some(GameStateResult {
                pop: true,
                game_state: None
            })
        }

        if is_key_pressed(KeyCode::Enter) {
            return Some(GameStateResult {
                pop: true,
                game_state: Some(Box::new(LevelGameState::new(self.sound_store.clone())))
            })
        }

        return None;
    }
    
    fn draw(&self) {
        clear_background(Color::from_rgba(0, 10, 35, 255));
    
        let text = match self.won {
            true => "You won :)",
            false => "You lost :(",
        };

        let center_paused = get_text_center(text, Option::None, 70, 1.0, 0.0);
        let height_paused = measure_text(text, Option::None, 70, 1.0);
        draw_text_ex(
            text,
            screen_width() / 2.0 - center_paused.x,
            screen_height() / 2.0 - center_paused.y,
            TextParams {
                font_size: 70,
                rotation: 0.0,
                ..Default::default()
            },
        );

        let center_continue = get_text_center("To play again press {enter} to exit press {escape}", Option::None, 20, 1.0, 0.0);
        draw_text_ex(
            "To play again press {enter} to exit press {escape}",
            screen_width() / 2.0 - center_continue.x,
            screen_height() / 2.0 - center_continue.y + height_paused.height + 5.0,
            TextParams {
                font_size: 20,
                rotation: 0.0,
                ..Default::default()
            },
        );
    }
}