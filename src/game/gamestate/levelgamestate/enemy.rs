use macroquad::prelude::*;

pub(super) struct Enemy {
    pub(super) bounding_box: Rect,
    pub(super) collision_layer: i32,
    pub(super) direction: i32,
    pub(super) is_alive: bool,
    time_since_last_shot: f32,
    shoot_time_offset: f32,
    can_shoot: bool,
    texture: Texture2D,
}

impl Enemy {
    pub(super) const ENEMY_SPEED: f32 = 200.0;
    pub(super) const ENEMY_SIZE: f32 = 20.0;

    pub(super) fn new(pos_x: f32, pos_y: f32, shoot_time_offset: f32) -> Self {
        Self {
            direction: 1,
            collision_layer: 2,
            bounding_box: Rect::new(pos_x, pos_y, Self::ENEMY_SIZE, Self::ENEMY_SIZE),
            is_alive: true,
            time_since_last_shot: 0.,
            shoot_time_offset,
            can_shoot: false,
            texture: Texture2D::from_file_with_format(
                include_bytes!("../../../../assets/green.png"),
                Some(ImageFormat::Png),
            ),
        }
    }

    pub(super) fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.bounding_box.x,
            self.bounding_box.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: self.bounding_box.w,
                    y: self.bounding_box.h,
                }),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    pub(super) fn update<F>(&mut self, delta_time: f32, change_direction: bool, mut spawn_bullet: F)
    where
        F: FnMut(f32, f32) -> (),
    {
        if change_direction {
            self.direction *= -1;
        }
        self.bounding_box.x += self.direction as f32 * Self::ENEMY_SPEED * delta_time;

        self.time_since_last_shot += delta_time;
        if !self.can_shoot && self.time_since_last_shot > self.shoot_time_offset {
            self.time_since_last_shot = 0.;
            self.can_shoot = true;
        }
        if self.can_shoot && self.time_since_last_shot >= 1. {
            self.time_since_last_shot = 0.;
            let random = rand::gen_range(0, 100);
            if random < 15 {
                spawn_bullet(
                    self.bounding_box.x + self.bounding_box.w / 2.0,
                    self.bounding_box.y + self.bounding_box.h + 0.1,
                );
            }
        }
    }
}
