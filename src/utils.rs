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

pub fn conf() -> Conf {
    Conf {
        window_title: String::from("Top down shooter"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        icon: Some(miniquad::conf::Icon {
            small: [0; 1024],
            medium: [0; 4096],
            big: [0; 16384],
        }),
        ..Default::default()
    }
}
