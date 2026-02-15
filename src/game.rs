use gamestate::levelgamestate::LevelGameState;
use macroquad::window::next_frame;
use std::rc::Rc;

mod gamestate;
mod soundstore;

pub struct Game {
    state: Vec<Box<dyn gamestate::GameState>>,
    sound_store: Rc<soundstore::SoundStore>
}

impl Game {
    pub fn new() -> Self {
         Self {
            state: Vec::new(),
            sound_store: Rc::new(soundstore::SoundStore::new())
        }
    }    

    pub async fn run(&mut self) {
        // TODO this needs to be the menu

        self.state.push(Box::new(LevelGameState::new(self.sound_store.clone())));

        while let Some(state) = self.state.last_mut() {
            loop {
                let result = state.update();

                match result {
                    Some(game_state) => {
                        if game_state.pop {
                            self.state.pop();
                        }
                        if let Some(new_state) = game_state.game_state {
                            self.state.push(new_state);
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