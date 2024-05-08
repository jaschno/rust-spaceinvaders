use gamestate::levelgamestate::LevelGameState;
use macroquad::window::next_frame;

mod gamestate;
mod soundstore;

pub struct Game {
    state: Vec<Box<dyn gamestate::GameState>>
}

impl Game {
    pub fn new() -> Self {
         Self {
            state: Vec::new()
        }
    }    

    pub async fn run(&mut self) {
        // TODO this needs to be the menu

        self.state.push(Box::new(LevelGameState::new()));

        while self.state.len() > 0 {
            let state = self.state.last_mut().unwrap();

            loop {
                let result = state.update();

                match result {
                    Some(game_state) => {
                        if game_state.pop {
                            self.state.pop();
                        }
                        if game_state.game_state.is_some() {
                            self.state.push(game_state.game_state.unwrap());
                        }
                        break;
                    },
                    None => (),
                }
                
                state.draw();

                next_frame().await;
            }
        }
    }
}