pub mod finalgamestate;
pub mod levelgamestate;
pub mod pausegamestate;

pub struct GameStateResult {
    pub pop: bool,
    pub game_state: Option<Box<dyn GameState>>
}

pub trait GameState {
    fn update(&mut self) -> Option<GameStateResult>;
    fn draw(&self);
}