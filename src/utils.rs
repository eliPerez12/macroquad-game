use macroquad::prelude::*;

// Checks what type of os is running the game

pub fn is_windows() -> bool {
    #[cfg(target_os = "windows")]
    return true;
    #[cfg(not(target_os = "windows"))]
    return false;
}

pub fn draw_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color)
}
