use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color)
}

fn default_camera_zoom() -> Vec2 {
    vec2(1.0 / screen_width(), 1.0 / screen_height())
}

#[macroquad::main(conf)]
async fn main() {
    let rect1 = Rect {
        x: 0.0,
        y: 0.0,
        w: 50.0,
        h: 200.0,
    };
    let mut rotation: f32 = 0.0;

    let mut camera = Camera2D {
        zoom: default_camera_zoom(),
        ..Default::default()
    };

    dbg!(screen_width(), screen_height());

    // Main game loop
    loop {
        clear_background(BLACK);
        rotation += 0.8;
        camera.zoom *= 1.002;

        camera.rotation = rotation;

        // Draw in world space
        set_camera(&camera);
        draw_rect(&rect1, RED);

        // Draw in screen space
        set_default_camera();

        next_frame().await;
    }
}
