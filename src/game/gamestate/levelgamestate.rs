use macroquad::{miniquad::date, prelude::*};

use crate::game::soundstore::SoundStore;

use super::{finalgamestate::FinalGameState, pausegamestate::PauseGameState, GameState, GameStateResult};

pub struct LevelGameState {
    world: World
}

impl LevelGameState {
    pub fn new() -> Self {
        let result = Self {
            world: World::new()
        };
        result.world.sound_store.play_background_music();
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

        if self.world.enemies.is_empty() {
            self.world.sound_store.play_win_sound();
            self.world.sound_store.stop_background_music();
            return Some(super::GameStateResult {
                pop: true,
                game_state: Some(Box::new(FinalGameState::new(true)))
            })
        }

        if !self.world.player.is_alive {
            self.world.sound_store.play_dead_sound();
            self.world.sound_store.stop_background_music();
            return Some(super::GameStateResult {
                pop: true,
                game_state: Some(Box::new(FinalGameState::new(false)))
            })
        }

        self.world.update(get_frame_time());

        return None;
    }
    
    fn draw(&self) {
        clear_background(Color::from_rgba(0, 10, 35, 0));
        self.world.draw();
    }
}

struct World {
    player:  Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    sound_store: SoundStore
}

impl World {
    fn new() -> World {
        rand::srand(date::now() as _);

        let mut instance = Self {
            player: Player::new(100.0, screen_height() - 50.0),
            bullets: Vec::new(),
            enemies: Vec::new(),
            sound_store: SoundStore::new()
        };
        instance.arrange_enemies(rand::gen_range(3, 5), rand::gen_range(5, 9));
        return instance;
    }

    fn arrange_enemies(&mut self, enemy_rows: i32, enemy_columns: i32) {
        let space_between: f32 = 35.0;
        let space_top: f32 = 60.0;
        let space_sides: f32 = 40.0;

        for row in 0..enemy_rows {
            for column in 0..enemy_columns {

                let pos_y = space_top + row as f32 * Enemy::ENEMY_SIZE + row as f32 * space_between;
                let pos_x = space_sides + column as f32 * Enemy::ENEMY_SIZE + column as f32 * space_between;
                let enemy = Enemy::new(pos_x, pos_y, rand::gen_range(0., 1.));

                self.enemies.push(enemy);
            }
        }
    }

    fn draw(&self) {
        self.player.draw();
        for bullet in &self.bullets {
            bullet.draw();
        }
        for enemy in &self.enemies {
            enemy.draw();
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.player.update(delta_time, |pos_x: f32, pos_y: f32| {
            self.bullets.push(Bullet::new(pos_x  - Bullet::BULLET_SIZE / 2.0, pos_y, 2,-1));
        });

        let mut change_direction = false;
        for enemy in &self.enemies {
            if enemy.direction == -1 && enemy.bounding_box.x < 40.0
            || enemy.direction == 1 && enemy.bounding_box.x + Enemy::ENEMY_SIZE > screen_width() - 40.0 {
                change_direction = true;
                break;
            }
        }

        for bullet in &mut self.bullets {
            bullet.update(delta_time);
        }
        for enemy in &mut self.enemies {
            enemy.update(delta_time, change_direction, |pos_x: f32, pos_y: f32| {
                self.bullets.push(Bullet::new(pos_x  - Bullet::BULLET_SIZE / 2.0, pos_y, 1, 1));
            });
        }

        self.check_collisions();

        self.bullets.retain(|bullet| {
            bullet.is_alive
        });
        self.enemies.retain(|enemy| {
            enemy.is_alive
        });
    }

    fn check_collisions(&mut self) {
        for bullet in &mut self.bullets {
            for enemy in &mut self.enemies {
                if bullet.collision_layer == enemy.collision_layer && enemy.bounding_box.overlaps(&bullet.bounding_box) {
                    enemy.is_alive = false;
                    bullet.is_alive = false;
                    self.sound_store.play_hit_sound();
                }
            }
            if bullet.collision_layer == self.player.collision_layer && self.player.bounding_box.overlaps(&bullet.bounding_box) {
                bullet.is_alive = false;
                self.player.is_alive = false;
                self.sound_store.play_hit_sound();
            }
        }
    }
}

struct Player {
    collision_layer: i32,
    is_alive: bool,
    bounding_box: Rect,
    texture: Texture2D
}

impl Player {
    const PLAYER_SPEED: f32 = 300.0;
    const PLAYER_SIZE: f32 = 25.0;

    fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            collision_layer: 1,
            is_alive: true,
            bounding_box: Rect::new(pos_x, pos_y, Self::PLAYER_SIZE, Self::PLAYER_SIZE),
            texture: Texture2D::from_file_with_format(include_bytes!("..\\..\\..\\assets\\player.png"), Some(ImageFormat::Png))
        }
    }

    fn draw(&self) {
        draw_texture_ex(&self.texture, self.bounding_box.x, self.bounding_box.y, WHITE, DrawTextureParams { 
            dest_size: Some(Vec2 { 
                x: self.bounding_box.w, 
                y: self.bounding_box.h 
            }), 
            source: None, 
            rotation: 0., 
            flip_x: false, 
            flip_y: false, 
            pivot: None });
    }

    fn update<F>(&mut self, delta_time: f32, mut spawn_bullet: F) 
    where F: FnMut(f32, f32) -> () 
    {
        let mut move_x = 0.0;
        if is_key_down(KeyCode::Right) {
            move_x += Self::PLAYER_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            move_x -= Self::PLAYER_SPEED;
        }
        self.bounding_box.x += move_x * delta_time;
        if self.bounding_box.x < 0.0 {
            self.bounding_box.x = 0.0;
        }
        if self.bounding_box.x + Player::PLAYER_SIZE > screen_width() {
            self.bounding_box.x = screen_width() - Player::PLAYER_SIZE;
        }

        if is_key_pressed(KeyCode::Space) {
            spawn_bullet(self.bounding_box.x + self.bounding_box.w / 2.0, self.bounding_box.y);
        }
    }
}

struct Enemy {
    bounding_box: Rect,
    collision_layer: i32,
    direction: i32,
    is_alive: bool,
    time_since_last_shot: f32,
    shoot_time_offset: f32,
    can_shoot: bool,
    texture: Texture2D
}

impl Enemy {
    const ENEMY_SPEED: f32 = 200.0;
    const ENEMY_SIZE: f32 = 20.0;

    fn new(pos_x: f32, pos_y: f32, shoot_time_offset: f32) -> Self {
        Self { 
            direction: 1,
            collision_layer: 2,
            bounding_box: Rect::new(pos_x, pos_y, Self::ENEMY_SIZE, Self::ENEMY_SIZE),
            is_alive: true,
            time_since_last_shot: 0.,
            shoot_time_offset,
            can_shoot: false,
            texture: Texture2D::from_file_with_format(include_bytes!("..\\..\\..\\assets\\green.png"), Some(ImageFormat::Png))

        }
    }

    fn draw(&self) {
        draw_texture_ex(&self.texture, self.bounding_box.x, self.bounding_box.y, WHITE, DrawTextureParams { 
            dest_size: Some(Vec2 { 
                x: self.bounding_box.w, 
                y: self.bounding_box.h 
            }), 
            source: None, 
            rotation: 0., 
            flip_x: false, 
            flip_y: false, 
            pivot: None });
    }

    fn update<F>(&mut self, delta_time: f32, change_direction: bool, mut spawn_bullet: F) 
    where F: FnMut(f32, f32) -> () {
        if change_direction {
            self.direction *= -1;
        }
        self.bounding_box.x += self.direction as f32 * Enemy::ENEMY_SPEED * delta_time;

        self.time_since_last_shot += delta_time;
        if self.can_shoot == false && self.time_since_last_shot > self.shoot_time_offset {
            self.time_since_last_shot = 0.;
            self.can_shoot = true;
        }
        if self.can_shoot && self.time_since_last_shot >= 1. {
            self.time_since_last_shot = 0.;
            let random = rand::gen_range(0, 100);
            if random < 15 {
                spawn_bullet(self.bounding_box.x + self.bounding_box.w / 2.0, self.bounding_box.y + self.bounding_box.h + 0.1 );
            }
        }
    }
}

struct Bullet {
    bounding_box: Rect,
    collision_layer: i32,
    is_alive: bool,
    direction: i32
}

impl Bullet {
    const BULLET_SPEED: f32 = 400.0;
    const BULLET_SIZE: f32 = 5.0;

    fn new(pos_x: f32, pos_y: f32, collision_layer: i32, direction: i32) -> Self {
        Self { 
            bounding_box: Rect::new(pos_x, pos_y, Self::BULLET_SIZE, Self::BULLET_SIZE),
            collision_layer,
            is_alive: true,
            direction
        }
    }

    fn draw(&self) {
        draw_rectangle(self.bounding_box.x, self.bounding_box.y, self.bounding_box.w, self.bounding_box.h, RED);
    }

    fn update(&mut self, delta_time: f32) {
        self.bounding_box.y += Self::BULLET_SPEED * delta_time * self.direction as f32;

        if self.bounding_box.y < 0.0 {
            self.is_alive = false;
        }
    }
}       