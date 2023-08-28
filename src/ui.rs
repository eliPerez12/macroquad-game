use crate::{player::*, utils::is_windows, camera::GameCamera};
use macroquad::prelude::*;

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

fn draw_rect_lines(rect: &Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color)
}

pub fn render_ui(player: &Player) {
    let health_bar = Rect {
        x: screen_width() / 20.0,
        y: screen_height() - screen_height() / 11.0,
        w: screen_width() / 5.0,
        h: screen_height() / 50.0,
    };

    let stamina_bar = Rect {
        x: screen_width() / 20.0,
        y: screen_height() - screen_height() / 15.0,
        w: screen_width() / 5.0,
        h: screen_height() / 80.0,
    };

    let filled_health_bar = {
        let mut filled_bar = health_bar.clone();
        filled_bar.w = player.health * filled_bar.w / 100.0;
        filled_bar
    };

    let filled_stamina_bar = {
        let mut filled_bar = stamina_bar.clone();
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

pub fn render_debug_ui(player: &Player, camera: &GameCamera) {
    let text_size = 50.0;
    draw_text(
        "Debug Menu v0.1.2",
        text_size/2.0,
        text_size/2.0 * 1.0 + text_size/4.0 * 1.0, 
        text_size,
        WHITE,
    );
    draw_text(
        &format!("FPS: {}", &get_fps().to_string()),
        text_size/2.0,
        text_size/2.0 * 2.0 + text_size/4.0 * 2.0, 
        text_size,
        WHITE,
    );
    draw_text(
        &format!("OS: {}",match is_windows() {
            true => "Windows",
            false => "Linux",
        }),
        text_size/2.0,
        text_size/2.0 * 3.0 + text_size/4.0 * 3.0, 
        text_size,
        WHITE,
    );
    draw_text(
        &format!("Health: {}", player.health),
        text_size/2.0,
        text_size/2.0 * 4.0 + text_size/4.0 * 4.0, 
        text_size,
        WHITE,
    );
    draw_text(
        &format!("Stamina: {}", player.stamina),
        text_size/2.0,
        text_size/2.0 * 5.0 + text_size/4.0 * 5.0, 
        text_size,
        WHITE,
    );
    draw_text(
        &format!("Player Pos: {}", (player.pos/8.0).floor() ),
        text_size/2.0,
        text_size/2.0 * 6.0 + text_size/4.0 * 6.0, 
        text_size,
        WHITE,
    );
    draw_text(
        &format!("Aiming at: {}", (camera.screen_to_world(mouse_position().into())/8.0).floor() ),
        text_size/2.0,
        text_size/2.0 * 7.0 + text_size/4.0 * 7.0, 
        text_size,
        WHITE,
    );
}

