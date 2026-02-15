use macroquad::prelude::*;

pub(super) struct Player {
    pub(super) collision_layer: i32,
    pub(super) is_alive: bool,
    pub(super) bounding_box: Rect,
    texture: Texture2D,
}

impl Player {
    const PLAYER_SPEED: f32 = 300.0;
    const PLAYER_SIZE: f32 = 25.0;

    pub(super) fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            collision_layer: 1,
            is_alive: true,
            bounding_box: Rect::new(pos_x, pos_y, Self::PLAYER_SIZE, Self::PLAYER_SIZE),
            texture: Texture2D::from_file_with_format(
                include_bytes!("../../../../assets/player.png"),
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

    pub(super) fn update<F>(&mut self, delta_time: f32, mut spawn_bullet: F)
    where
        F: FnMut(f32, f32) -> (),
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
        if self.bounding_box.x + Self::PLAYER_SIZE > screen_width() {
            self.bounding_box.x = screen_width() - Self::PLAYER_SIZE;
        }

        if is_key_pressed(KeyCode::Space) {
            spawn_bullet(self.bounding_box.x + self.bounding_box.w / 2.0, self.bounding_box.y);
        }
    }
}
