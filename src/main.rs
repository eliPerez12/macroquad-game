use camera::*;
use macroquad::prelude::*;
use player::*;

mod camera;
mod player;

fn get_src_rect(atlas_grid_x: i32, atlas_grid_y: i32) -> Rect {
    Rect {
        x: atlas_grid_x as f32 * 8.0,
        y: atlas_grid_y as f32 * 8.0,
        w: 8.0,
        h: 8.0,
    }
}

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

fn draw_rect_lines(rect: &Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color)
}

async fn load_texture(path: &str) -> Result<Texture2D, macroquad::Error> {
    let texture = Texture2D::from_image(&load_image(path).await?);
    texture.set_filter(FilterMode::Nearest);
    Ok(texture)
}

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera {
        ..Default::default()
    };

    let _tiles_sheet = load_texture("assets/tiles.png").await.unwrap();
    let player_sprite = load_texture("assets/soldier.png").await.unwrap();
    let example_world = load_texture("assets/sample.png").await.unwrap();

    let mut player = Player::new();

    // Main game loop
    loop {
        // Update Game
        player.handle_player_movements();

        camera.handle_controls();
        camera.pan_to_target(player.pos);

        // Draw in world space
        set_camera(&mut camera);
        clear_background(BLACK);

        // Draws example world
        draw_texture(&example_world, 0.0, 0.0, WHITE);

        // Draw player shadow
        draw_circle(
            player.pos.x + 0.50,
            player.pos.y + 0.50,
            3.2,
            Color::from_rgba(0, 0, 0, 70),
        );

        // Draw player
        draw_texture_ex(
            &player_sprite,
            player.pos.x - 5.5,
            player.pos.y - 5.5,
            WHITE,
            DrawTextureParams {
                rotation: player.angle,
                pivot: Some(player.pos),
                dest_size: Some(Vec2::new(11.0, 11.0)),
                ..Default::default()
            },
        );

        // Draw in screen space
        set_default_camera();
        draw_text(&get_fps().to_string(), 50.0, 50.0, 100.0, WHITE);

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

        draw_rect(&filled_health_bar, Color::from_rgba(255, 30, 30, 180));
        draw_rect_lines(&health_bar, 3.0, BLACK);

        draw_rect(&filled_stamina_bar, Color::from_rgba(50, 50, 50, 180));
        draw_rect_lines(&stamina_bar, 3.0, BLACK);

        next_frame().await;
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}
