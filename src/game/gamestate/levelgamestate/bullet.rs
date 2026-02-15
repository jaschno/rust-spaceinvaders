use macroquad::prelude::*;

pub(super) struct Bullet {
    pub(super) bounding_box: Rect,
    pub(super) collision_layer: i32,
    pub(super) is_alive: bool,
    direction: i32,
}

impl Bullet {
    const BULLET_SPEED: f32 = 400.0;
    pub(super) const BULLET_SIZE: f32 = 5.0;

    pub(super) fn new(pos_x: f32, pos_y: f32, collision_layer: i32, direction: i32) -> Self {
        Self {
            bounding_box: Rect::new(pos_x, pos_y, Self::BULLET_SIZE, Self::BULLET_SIZE),
            collision_layer,
            is_alive: true,
            direction,
        }
    }

    pub(super) fn draw(&self) {
        draw_rectangle(
            self.bounding_box.x,
            self.bounding_box.y,
            self.bounding_box.w,
            self.bounding_box.h,
            RED,
        );
    }

    pub(super) fn update(&mut self, delta_time: f32) {
        self.bounding_box.y += Self::BULLET_SPEED * delta_time * self.direction as f32;

        if self.bounding_box.y < 0.0 || self.bounding_box.y > screen_height() {
            self.is_alive = false;
        }
    }
}
