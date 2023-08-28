use macroquad::prelude::*;

// Checks what type of os is running the game

#[cfg(target_os = "windows")]
pub fn is_windows() -> bool { true }

#[cfg(target_os = "linux")]
pub fn is_windows() -> bool { false }

pub fn draw_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color)
}