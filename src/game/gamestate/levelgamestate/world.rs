use macroquad::{miniquad::date, prelude::*};

use crate::game::soundstore::SoundStore;

use super::{bullet::Bullet, enemy::Enemy, player::Player};

pub(super) struct World {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
}

impl World {
    pub(super) fn new() -> Self {
        rand::srand(date::now() as _);

        let mut instance = Self {
            player: Player::new(100.0, screen_height() - 50.0),
            bullets: Vec::new(),
            enemies: Vec::new(),
        };
        instance.arrange_enemies(rand::gen_range(3, 5), rand::gen_range(5, 9));
        instance
    }

    pub(super) fn has_no_enemies(&self) -> bool {
        self.enemies.is_empty()
    }

    pub(super) fn is_player_alive(&self) -> bool {
        self.player.is_alive
    }

    pub(super) fn draw(&self) {
        self.player.draw();
        for bullet in &self.bullets {
            bullet.draw();
        }
        for enemy in &self.enemies {
            enemy.draw();
        }
    }

    pub(super) fn update(&mut self, delta_time: f32, sound_store: &SoundStore) {
        self.player.update(delta_time, |pos_x: f32, pos_y: f32| {
            self.bullets
                .push(Bullet::new(pos_x - Bullet::BULLET_SIZE / 2.0, pos_y, 2, -1));
        });

        let mut change_direction = false;
        for enemy in &self.enemies {
            if enemy.direction == -1 && enemy.bounding_box.x < 40.0
                || enemy.direction == 1
                    && enemy.bounding_box.x + Enemy::ENEMY_SIZE > screen_width() - 40.0
            {
                change_direction = true;
                break;
            }
        }

        for bullet in &mut self.bullets {
            bullet.update(delta_time);
        }
        for enemy in &mut self.enemies {
            enemy.update(delta_time, change_direction, |pos_x: f32, pos_y: f32| {
                self.bullets
                    .push(Bullet::new(pos_x - Bullet::BULLET_SIZE / 2.0, pos_y, 1, 1));
            });
        }

        self.check_collisions(sound_store);

        self.bullets.retain(|bullet| bullet.is_alive);
        self.enemies.retain(|enemy| enemy.is_alive);
    }

    fn arrange_enemies(&mut self, enemy_rows: i32, enemy_columns: i32) {
        let space_between: f32 = 35.0;
        let space_top: f32 = 60.0;
        let space_sides: f32 = 40.0;

        for row in 0..enemy_rows {
            for column in 0..enemy_columns {
                let pos_y =
                    space_top + row as f32 * Enemy::ENEMY_SIZE + row as f32 * space_between;
                let pos_x =
                    space_sides + column as f32 * Enemy::ENEMY_SIZE + column as f32 * space_between;
                let enemy = Enemy::new(pos_x, pos_y, rand::gen_range(0., 1.));

                self.enemies.push(enemy);
            }
        }
    }

    fn check_collisions(&mut self, sound_store: &SoundStore) {
        for bullet in &mut self.bullets {
            for enemy in &mut self.enemies {
                if bullet.collision_layer == enemy.collision_layer
                    && enemy.bounding_box.overlaps(&bullet.bounding_box)
                {
                    enemy.is_alive = false;
                    bullet.is_alive = false;
                    sound_store.play_hit_sound();
                }
            }
            if bullet.collision_layer == self.player.collision_layer
                && self.player.bounding_box.overlaps(&bullet.bounding_box)
            {
                bullet.is_alive = false;
                self.player.is_alive = false;
                sound_store.play_hit_sound();
            }
        }
    }
}
