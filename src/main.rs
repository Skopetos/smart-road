mod constants;
mod intersection;
mod vehicle;

use macroquad::prelude::*;
use macroquad::rand::gen_range;
use constants::*;
use vehicle::{Direction, Route, Vehicle, spawn_pos};

// Minimum gap between two vehicles in the same lane at the spawn point.
// Prevents vehicles being created on top of each other when keys are spammed.
const MIN_SPAWN_GAP: f32 = CAR_LENGTH + SAFE_DIST * 3.0;

// How often (in seconds) a new vehicle is spawned while R is held.
const R_INTERVAL: f32 = 0.9;

fn window_conf() -> Conf {
    Conf {
        window_title: "Smart Road".to_owned(),
        window_width:  WIN_W as i32,
        window_height: WIN_H as i32,
        ..Default::default()
    }
}

fn random_route() -> Route {
    match gen_range(0u32, 3) {
        0 => Route::Right,
        1 => Route::Straight,
        _ => Route::Left,
    }
}

fn random_direction() -> Direction {
    match gen_range(0u32, 4) {
        0 => Direction::North,
        1 => Direction::South,
        2 => Direction::East,
        _ => Direction::West,
    }
}

/// Tries to add a vehicle at the spawn point for (origin, route).
/// Does nothing if an existing vehicle is still too close to that point.
fn try_spawn(vehicles: &mut Vec<Vehicle>, id: &mut u32, origin: Direction, route: Route) {
    let pos = spawn_pos(origin, route);
    let blocked = vehicles.iter().any(|v| {
        v.origin == origin && v.route == route
            && (v.pos - pos).length() < MIN_SPAWN_GAP
    });
    if !blocked {
        vehicles.push(Vehicle::new(*id, origin, route, SPEED_NORMAL));
        *id += 1;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grass = Color { r: 0.13, g: 0.40, b: 0.13, a: 1.0 };

    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut next_id:  u32  = 0;
    let mut r_timer:  f32  = R_INTERVAL; // start ready so first R press spawns immediately

    loop {
        let dt  = get_frame_time().min(0.05);
        let now = get_time();

        // ── Arrow keys: spawn one vehicle from the matching arm ────────────────
        // Up    → vehicle enters from South heading north
        // Down  → vehicle enters from North heading south
        // Right → vehicle enters from West heading east
        // Left  → vehicle enters from East heading west
        if is_key_pressed(KeyCode::Up) {
            try_spawn(&mut vehicles, &mut next_id, Direction::South, random_route());
        }
        if is_key_pressed(KeyCode::Down) {
            try_spawn(&mut vehicles, &mut next_id, Direction::North, random_route());
        }
        if is_key_pressed(KeyCode::Right) {
            try_spawn(&mut vehicles, &mut next_id, Direction::West, random_route());
        }
        if is_key_pressed(KeyCode::Left) {
            try_spawn(&mut vehicles, &mut next_id, Direction::East, random_route());
        }

        // ── R key: continuously spawn random vehicles on a timer ───────────────
        if is_key_down(KeyCode::R) {
            r_timer += dt;
            if r_timer >= R_INTERVAL {
                r_timer = 0.0;
                try_spawn(&mut vehicles, &mut next_id, random_direction(), random_route());
            }
        } else {
            r_timer = R_INTERVAL; // reset so the next R press spawns immediately
        }

        // ── Esc: quit (stats window will be added in step 9) ──────────────────
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // ── Update ─────────────────────────────────────────────────────────────
        for v in &mut vehicles {
            v.update(dt, now);
        }
        vehicles.retain(|v| !v.is_done());

        // ── Draw ───────────────────────────────────────────────────────────────
        clear_background(grass);
        intersection::draw();
        for v in &vehicles {
            v.draw();
        }

        next_frame().await;
    }
}
