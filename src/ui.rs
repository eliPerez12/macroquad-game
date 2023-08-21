use macroquad::prelude::*;
use crate::player::*;

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
        PlayerStaminaState::Recovering => Color::from_rgba(100, 100, 100, 200)
    };
    
    draw_rect(&filled_health_bar, Color::from_rgba(255, 30, 30, 200));
    draw_rect_lines(&health_bar, 3.0, BLACK);
    
    draw_rect(&filled_stamina_bar, stamina_bar_color);
    draw_rect_lines(&stamina_bar, 3.0, BLACK);
}