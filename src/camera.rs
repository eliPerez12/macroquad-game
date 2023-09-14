#![allow(dead_code)]
use crate::world::TileMap;
use macroquad::prelude::*;

pub struct GameCamera {
    pub rotation: f32,
    pub zoom: Vec2,
    pub target_zoom: f32,
    pub target: Vec2,
    pub offset: Vec2,
    pub render_target: Option<RenderTarget>,
    pub viewport: Option<(i32, i32, i32, i32)>,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn handle_controls(&mut self) {
        #[cfg(target_os = "linux")]
        {
            self.target_zoom *= mouse_wheel().1 / 10.0 + 1.0;
        }
        #[cfg(target_os = "windows")]
        {
            self.target_zoom *= mouse_wheel().1 / 1200.0 + 1.0;
        }

        let (max_zoom, min_zoom) = (6.0, 30.0);

        self.target_zoom = self.target_zoom.min(min_zoom);
        self.target_zoom = self.target_zoom.max(max_zoom);

        self.set_camera_zoom();
    }

    pub fn world_to_screen(&self, point: Vec2) -> Vec2 {
        let mat = self.matrix();
        let transform = mat.transform_point3(vec3(point.x, point.y, 0.));

        vec2(
            (transform.x / 2.0 + 0.5) * screen_width(),
            (0.5 - transform.y / 2.0) * screen_height(),
        )
    }

    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        let point = vec2(
            point.x / screen_width() * 2. - 1.0,
            1.0 - point.y / screen_height() * 2.0,
        );
        let inv_mat = self.matrix().inverse();
        let transform = inv_mat.transform_point3(vec3(point.x, point.y, 0.0));

        vec2(transform.x, transform.y)
    }

    pub fn set_default_camera_zoom(&mut self) {
        self.zoom = Self::default_camera_zoom()
    }

    pub fn set_camera_zoom(&mut self) {
        self.zoom = Self::default_camera_zoom();
        self.zoom *= self.target_zoom;
    }

    fn default_camera_zoom() -> Vec2 {
        vec2(1.0 / screen_width(), 1.0 / screen_height())
    }

    // Moves camera to target slowly
    pub fn pan_to_target(&mut self, target: Vec2) {
        const PAN_SPEED: f32 = 12.0; // Bigger number means slower pan
        let camera_dist_from_player = self.target - target;
        self.target -= camera_dist_from_player / PAN_SPEED * get_frame_time() * 60.0;
    }

    pub fn get_visible_tiles(&self, world: &TileMap) -> Vec<(u16, u16)> {
        let mut visible_tiles = vec![];
        let top_left = self.screen_to_world(Vec2::ZERO);
        let bottom_right = self.screen_to_world(Vec2::new(screen_width(), screen_height()));
        let (top_grid_x, top_grid_y) = ((top_left.x as u16 / 8), top_left.y as u16 / 8);
        let (bottom_grid_x, bottom_grid_y) = (
            (bottom_right.x as u16 / 8) + 1,
            bottom_right.y as u16 / 8 + 1,
        );
        visible_tiles.reserve(((bottom_grid_x - top_grid_x) * (bottom_grid_y - top_grid_y)).into());

        for y in top_grid_y.min(world.height)..bottom_grid_y.min(world.height) {
            for x in top_grid_x.min(world.width)..bottom_grid_x.min(world.width) {
                visible_tiles.push((x, y));
            }
        }
        visible_tiles
    }
}

impl Camera for GameCamera {
    fn matrix(&self) -> Mat4 {
        let mat_origin = Mat4::from_translation(vec3(-self.target.x, -self.target.y, 0.0));
        let mat_rotation = Mat4::from_axis_angle(vec3(0.0, 0.0, 1.0), self.rotation.to_radians());
        let invert_y = if self.render_target.is_some() {
            1.0
        } else {
            -1.0
        };
        let mat_scale = Mat4::from_scale(vec3(self.zoom.x, self.zoom.y * invert_y, 1.0));
        let mat_translation = Mat4::from_translation(vec3(self.offset.x, self.offset.y, 0.0));

        mat_translation * mat_scale * mat_rotation * mat_origin
    }

    fn depth_enabled(&self) -> bool {
        false
    }

    fn render_pass(&self) -> Option<miniquad::RenderPass> {
        self.render_target.as_ref().map(|rt| rt.render_pass)
    }

    fn viewport(&self) -> Option<(i32, i32, i32, i32)> {
        self.viewport
    }
}

impl Default for GameCamera {
    fn default() -> GameCamera {
        GameCamera {
            zoom: GameCamera::default_camera_zoom(),
            offset: vec2(0., 0.),
            target: vec2(0., 0.),
            rotation: 0.,
            target_zoom: 15.0,
            render_target: None,
            viewport: None,
        }
    }
}
