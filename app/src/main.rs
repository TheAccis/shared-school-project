use macroquad::prelude::*;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Car {
    x: f32,
    y: f32,
    dir: Direction,
    color: Color,
    speed: f32,
}

#[macroquad::main("Traffic Intersection Demo")]
async fn main() {
    let font = load_ttf_font("res/Inter_18pt-Medium.ttf").await.unwrap();

    let mut cars: Vec<Car> = vec![];
    let spawn_prob = 2; // %

    // Текущее состояние режима управления
    let mut adaptive_mode = false;

    fastrand::seed(0);

    loop {
        clear_background(BLACK);

        // Рисуем окно перекрёстка (очень упрощённо)
        draw_crossroad();

        // Генерация машин
        if fastrand::i32(0..100) < spawn_prob {
            cars.push(spawn_car());
        }

        // Обновляем позиции машин
        for car in cars.iter_mut() {
            match car.dir {
                Direction::North => car.y -= car.speed,
                Direction::South => car.y += car.speed,
                Direction::West => car.x -= car.speed,
                Direction::East => car.x += car.speed,
            }
        }

        // Рисуем машины
        for car in cars.iter() {
            draw_rectangle(car.x, car.y, 20.0, 10.0, car.color);
        }

        // Удаляем машины, которые выехали за экран
        cars.retain(|car| car.x >= -50.0 && car.x <= screen_width() + 50.0
                        && car.y >= -50.0 && car.y <= screen_height() + 50.0);

        // Текстовая индикация режима
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

        // Обработка клавиш
        if is_key_pressed(KeyCode::Space) {
            adaptive_mode = !adaptive_mode;
        }
        if is_key_pressed(KeyCode::R) {
            // Для начала просто выводим сообщение
            println!("Симуляция перезапущена!");
        }

        next_frame().await;
    }
}

// Функция генерации машины в случайном направлении
fn spawn_car() -> Car {
    let color = Color::from_rgba(
        fastrand::u8(0..=255),
        fastrand::u8(0..=255),
        fastrand::u8(0..=255),
        255
    );

    let w = screen_width();
    let h = screen_height();
    let speed = 2.0;

    match fastrand::i32(0..4) {
        0 => Car { x: w/2.0 - 10.0, y: h, dir: Direction::North, color, speed },
        1 => Car { x: w/2.0 - 10.0, y: -10.0, dir: Direction::South, color, speed },
        2 => Car { x: w, y: h/2.0 - 5.0, dir: Direction::West, color, speed },
        3 => Car { x: -20.0, y: h/2.0 - 5.0, dir: Direction::East, color, speed },
        _ => unreachable!()
    }
}

// Очень простая визуализация перекрёстка
fn draw_crossroad() {
    let w = screen_width();
    let h = screen_height();

    let line_thickness = 40.0;

    // Горизонтальная дорога
    draw_rectangle(0.0, h/2.0 - line_thickness/2.0, w, line_thickness, GRAY);
    // Вертикальная дорога
    draw_rectangle(w/2.0 - line_thickness/2.0, 0.0, line_thickness, h, GRAY);
}