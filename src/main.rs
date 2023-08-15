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

// draw texture in grid

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color)
}

fn default_camera_zoom() -> Vec2 {
    vec2(1.0 / screen_width(), 1.0 / screen_height())
}

#[macroquad::main(conf)]
async fn main() {

    let mut camera = Camera2D {
        zoom: default_camera_zoom(),
        ..Default::default()
    };

    let mut camera_zoom = 1.0;

    let tiles_sheet = Texture2D::from_file_with_format(
        include_bytes!("assets/jawbreaker_tiles.png"),
        Some(ImageFormat::Png));
    tiles_sheet.set_filter(FilterMode::Nearest);
    
    // Main game loop
    loop {
        clear_background(BLACK);

        let camera_speed = 1.0;

        if is_key_down(KeyCode::W) {
            camera.target.y -= camera_speed;
        }
        if is_key_down(KeyCode::S) {
            camera.target.y += camera_speed;
        }
        if is_key_down(KeyCode::A) {
            camera.target.x -= camera_speed;
        }
        if is_key_down(KeyCode::D) {
            camera.target.x += camera_speed;
        }

        if is_key_down(KeyCode::Q) {
            camera_zoom *= 1.01;
        }
        if is_key_down(KeyCode::E) {
            camera_zoom /= 1.01;
        }

    
        draw_texture_ex(&tiles_sheet, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(8.0, 8.0)),
            source: Some(Rect::new(7.0 * 8.0, 2.0 * 8.0, 8.0, 8.0)),
                ..Default::default()
            }
        );
        // Draw in screen space
        set_default_camera();
        draw_text(&get_fps().to_string(), 50.0, 50.0, 100.0, WHITE);

        next_frame().await;
    }
}
