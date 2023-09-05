use crate::{camera::GameCamera, player::*, utils::is_windows, world::TileMap};
use std::collections::VecDeque;
use macroquad::prelude::*;

pub struct FpsBarGraph {
    fps_record: VecDeque<i32>,
    highest_fps: f32
}

impl FpsBarGraph {
    pub fn new() -> Self{
        let mut fps_record = VecDeque::new();
        for _ in 0..100 {
            fps_record.push_front(0);
        }
        Self {
            fps_record,
            highest_fps: 0.0,
        }
    }
    pub fn update(&mut self) {
        self.fps_record.push_back(get_fps());
        self.fps_record.pop_front();
        if get_fps() as f32 > self.highest_fps {
            self.highest_fps = get_fps() as f32;
        }
    }

    pub fn draw(&self) {
        let height = 80.0;
        let width = screen_width()/7.0;
        for (index, fps) in self.fps_record.iter().enumerate() {
            let fps = *fps as f32;
            let color = {
                if fps < 25.0 {
                    Color::new(1.0, 0.0, 0.0, 0.8)
                }
                else if fps < 40.0 {
                    Color::new(0.6, 1.0, 0.2, 0.8)
                }
                else {
                    Color::new(1.0, 1.0, 1.0, 0.8)
                }
            };
            let h = (fps/self.highest_fps) * height;
            //let y = 100.0 - h;
            let bw = width/self.fps_record.len() as f32; // Width of each bar
            draw_rectangle(screen_width() - width + index as f32 * bw, 0.0, bw, h, color);
        }
    }
}

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

fn draw_rect_lines(rect: &Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color)
}

pub fn render_ui(player: &Player) {
    let health_bar = Rect {
        x: screen_width() * 0.03,
        y: screen_height() - screen_height() / 12.0,
        w: screen_width() / 4.5,
        h: screen_height() / 45.0,
    };

    let stamina_bar = Rect {
        x: health_bar.x,
        y: health_bar.y + health_bar.h,
        w: health_bar.w,
        h: health_bar.h * 0.8,
    };

    let filled_health_bar = {
        let mut filled_bar = health_bar;
        filled_bar.w = player.health * filled_bar.w / 100.0;
        filled_bar
    };

    let filled_stamina_bar = {
        let mut filled_bar = stamina_bar;
        filled_bar.w = player.stamina * filled_bar.w / 100.0;
        filled_bar
    };

    let stamina_bar_color = match player.stamina_state {
        PlayerStaminaState::Normal => Color::from_rgba(60, 60, 60, 180),
        PlayerStaminaState::Recovering => Color::from_rgba(100, 100, 100, 200),
    };

    draw_rect(&filled_health_bar, Color::from_rgba(255, 30, 30, 200));
    draw_rect_lines(&health_bar, 3.0, BLACK);

    draw_rect(&filled_stamina_bar, stamina_bar_color);
    draw_rect_lines(&stamina_bar, 3.0, BLACK);
}

pub fn render_debug_ui(player: &Player, camera: &GameCamera, world: &TileMap) {
    let text_size = 45.0;
    let mut ui_stack = vec![];

    ui_stack.push(format!(
        "Debug Menu v0.1.2 ({} on {})",
        {
            #[cfg(debug_assertions)]
            {
                "Debug Build"
            }
            #[cfg(not(debug_assertions))]
            {
                "Release Build"
            }
        },
        {
            match is_windows() {
                true => "Windows",
                false => "Linux",
            }
        }
    ));
    ui_stack.push(format!("FPS: {}", &get_fps().to_owned()));
    ui_stack.push(format!(
        "OS: {}",
        match is_windows() {
            true => "Windows",
            false => "Linux",
        }
    ));

    ui_stack.push(format!("Health: {}", player.health.round()));
    ui_stack.push(format!("Stamina: {}", player.stamina.round()));
    ui_stack.push(format!("Player Pos: {}", (player.pos / 8.0).floor()));
    ui_stack.push(format!(
        "Aiming at: {}",
        (camera.screen_to_world(mouse_position().into()) / 8.0).floor()
    ));

    ui_stack.push(format!(
        "Tiles visible: {}",
        camera.get_visible_tiles(&world).len()
    ));

    for (stack_pos, element) in ui_stack.iter().enumerate() {
        draw_text(
            element,
            text_size / 2.0,
            text_size / 2.0 * stack_pos as f32 + text_size / 4.0 * stack_pos as f32 + text_size,
            text_size,
            WHITE,
        );
    }
}
