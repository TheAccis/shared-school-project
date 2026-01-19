use macroquad::prelude::*;
use graphics::CarView;
use glam::Vec2;

#[macroquad::main("Traffic Intersection Demo")]
async fn main() {
    let font = load_ttf_font("res/Inter_18pt-Medium.ttf").await.unwrap();

    let mut car_views: Vec<CarView> = vec![];
    let spawn_prob = 2; // %
    let speed: f32 = 2.0;

    let mut adaptive_mode = false;

    fastrand::seed(0);

    loop {
        clear_background(BLACK);

        draw_crossroad();

        if fastrand::i32(0..100) < spawn_prob {
            car_views.push(CarView::new(speed));
        }

        for car_view in car_views.iter_mut() {
            car_view.car.moving = true;
            car_view.car.step();
        }

        for car_view in car_views.iter() {
            car_view.draw();
        }

        let screen_size = Vec2::new(screen_width(), screen_height());
        car_views.retain(|car_view| !car_view.car.is_outside(screen_size));

        let mode_text = if adaptive_mode {
            "Режим: Адаптивный"
        } else {
            "Режим: Обычный"
        };
        draw_text_ex(
            mode_text,
            20.0,
            30.0,
            TextParams {
                font: Some(&font),
                font_size: 30,
                color: BLACK,
                ..Default::default()
            },
        );

        if is_key_pressed(KeyCode::Space) {
            adaptive_mode = !adaptive_mode;
        }
        if is_key_pressed(KeyCode::R) {
            println!("Симуляция перезапущена!");
        }

        next_frame().await;
    }
}

fn draw_crossroad() {
    let w = screen_width();
    let h = screen_height();

    let line_thickness = 40.0;

    draw_rectangle(0.0, h/2.0 - line_thickness/2.0, w, line_thickness, GRAY);
    draw_rectangle(w/2.0 - line_thickness/2.0, 0.0, line_thickness, h, GRAY);
}