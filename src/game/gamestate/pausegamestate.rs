use macroquad::prelude::*;

use super::{GameState, GameStateResult};

pub struct PauseGameState {
}

impl GameState for PauseGameState {
    fn update(&mut self) -> Option<GameStateResult> {
        if is_key_pressed(KeyCode::Escape) {
            return Some(GameStateResult {
                pop: true,
                game_state: None
            })
        }

        if is_key_pressed(KeyCode::Enter) {
            return Some(GameStateResult {
                pop: true,
                game_state: None
            })
        }

        return None;
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(0, 10, 35, 255));
        
        let center_paused = get_text_center("Paused", Option::None, 70, 1.0, 0.0);
        let height_paused = measure_text("Paused", Option::None, 70, 1.0);
        draw_text_ex(
            "Paused",
            screen_width() / 2.0 - center_paused.x,
            screen_height() / 2.0 - center_paused.y,
            TextParams {
                font_size: 70,
                rotation: 0.0,
                ..Default::default()
            },
        );

        let center_continue = get_text_center("To continue press {enter}", Option::None, 20, 1.0, 0.0);
        draw_text_ex(
            "To continue press {enter}",
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