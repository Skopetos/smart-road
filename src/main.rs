mod constants;
mod intersection;
mod vehicle;

use macroquad::prelude::*;
use constants::{WIN_W, WIN_H, SPEED_NORMAL};
use vehicle::{Direction, Route, Vehicle};

fn window_conf() -> Conf {
    Conf {
        window_title: "Smart Road".to_owned(),
        window_width:  WIN_W as i32,
        window_height: WIN_H as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grass = Color { r: 0.13, g: 0.40, b: 0.13, a: 1.0 };

    // Spawn one test vehicle per direction × route to verify all 12 paths.
    let mut vehicles: Vec<Vehicle> = vec![
        Vehicle::new(0,  Direction::North, Route::Right,    SPEED_NORMAL),
        Vehicle::new(1,  Direction::North, Route::Straight, SPEED_NORMAL),
        Vehicle::new(2,  Direction::North, Route::Left,     SPEED_NORMAL),
        Vehicle::new(3,  Direction::South, Route::Right,    SPEED_NORMAL),
        Vehicle::new(4,  Direction::South, Route::Straight, SPEED_NORMAL),
        Vehicle::new(5,  Direction::South, Route::Left,     SPEED_NORMAL),
        Vehicle::new(6,  Direction::East,  Route::Right,    SPEED_NORMAL),
        Vehicle::new(7,  Direction::East,  Route::Straight, SPEED_NORMAL),
        Vehicle::new(8,  Direction::East,  Route::Left,     SPEED_NORMAL),
        Vehicle::new(9,  Direction::West,  Route::Right,    SPEED_NORMAL),
        Vehicle::new(10, Direction::West,  Route::Straight, SPEED_NORMAL),
        Vehicle::new(11, Direction::West,  Route::Left,     SPEED_NORMAL),
    ];

    let mut next_id: u32 = 12;
    let _ = next_id; // will be used in step 4

    loop {
        let dt  = get_frame_time().min(0.05); // cap delta to avoid spiral-of-death
        let now = get_time();

        clear_background(grass);
        intersection::draw();

        for v in &mut vehicles {
            v.update(dt, now);
            v.draw();
        }
        vehicles.retain(|v| !v.is_done());

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
