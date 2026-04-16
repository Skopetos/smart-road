use macroquad::prelude::*;
use crate::constants::*;
use std::f32::consts::PI;

// ── Colours ───────────────────────────────────────────────────────────────────
const ROAD_CLR: Color = Color { r: 0.31, g: 0.31, b: 0.31, a: 1.00 };
const INTER_CLR: Color = Color { r: 0.22, g: 0.22, b: 0.22, a: 1.00 };
const EDGE_CLR: Color = Color { r: 0.85, g: 0.85, b: 0.85, a: 0.80 };
const DIV_CLR: Color = Color { r: 1.00, g: 1.00, b: 1.00, a: 0.50 };
const CTR_CLR: Color = Color { r: 1.00, g: 0.85, b: 0.00, a: 1.00 };
const MARK_CLR: Color = Color { r: 1.00, g: 1.00, b: 1.00, a: 0.90 };
const STOP_CLR: Color = Color { r: 1.00, g: 1.00, b: 1.00, a: 1.00 };

// ── Public entry point ────────────────────────────────────────────────────────
pub fn draw() {
    draw_road_surfaces();
    draw_road_borders();
    draw_lane_dividers();
    draw_center_lines();
    draw_stop_lines();
    draw_lane_markers();
}

// ── Road surfaces ─────────────────────────────────────────────────────────────
fn draw_road_surfaces() {
    // Vertical road (north–south, full height)
    draw_rectangle(INTER_L, 0.0, ROAD_W, WIN_H, ROAD_CLR);
    // Horizontal road (west–east, full width)
    draw_rectangle(0.0, INTER_T, WIN_W, ROAD_W, ROAD_CLR);
    // Intersection box — slightly darker to distinguish it
    draw_rectangle(INTER_L, INTER_T, ROAD_W, ROAD_W, INTER_CLR);
}

// ── Outer kerb lines ──────────────────────────────────────────────────────────
fn draw_road_borders() {
    let t = 2.5;
    // Vertical road — left (west) and right (east) edges, excluding the intersection box
    draw_line(INTER_L, 0.0,    INTER_L, INTER_T, t, EDGE_CLR);
    draw_line(INTER_L, INTER_B, INTER_L, WIN_H,  t, EDGE_CLR);
    draw_line(INTER_R, 0.0,    INTER_R, INTER_T, t, EDGE_CLR);
    draw_line(INTER_R, INTER_B, INTER_R, WIN_H,  t, EDGE_CLR);
    // Horizontal road — top (north) and bottom (south) edges
    draw_line(0.0,    INTER_T, INTER_L, INTER_T, t, EDGE_CLR);
    draw_line(INTER_R, INTER_T, WIN_W,  INTER_T, t, EDGE_CLR);
    draw_line(0.0,    INTER_B, INTER_L, INTER_B, t, EDGE_CLR);
    draw_line(INTER_R, INTER_B, WIN_W,  INTER_B, t, EDGE_CLR);
}

// ── Dashed lane dividers ──────────────────────────────────────────────────────
fn draw_lane_dividers() {
    // Vertical dashed lines on the vertical road arms (not inside intersection)
    let div_xs = [
        INTER_L + LANE_W,        // 320 — between SB r and s
        INTER_L + LANE_W * 2.0,  // 360 — between SB s and l
        CENTER_X + LANE_W,       // 440 — between NB l and s
        CENTER_X + LANE_W * 2.0, // 480 — between NB s and r
    ];
    for &x in &div_xs {
        dash_v(x, 0.0,    INTER_T);
        dash_v(x, INTER_B, WIN_H);
    }

    // Horizontal dashed lines on the horizontal road arms
    let div_ys = [
        INTER_T + LANE_W,        // 320 — between WB r and s
        INTER_T + LANE_W * 2.0,  // 360 — between WB s and l
        CENTER_Y + LANE_W,       // 440 — between EB l and s
        CENTER_Y + LANE_W * 2.0, // 480 — between EB s and r
    ];
    for &y in &div_ys {
        dash_h(0.0,    INTER_L, y);
        dash_h(INTER_R, WIN_W,  y);
    }
}

// ── Solid yellow centre lines ─────────────────────────────────────────────────
fn draw_center_lines() {
    let t = 3.0;
    // Between southbound and northbound halves of vertical road
    draw_line(CENTER_X, 0.0,    CENTER_X, INTER_T, t, CTR_CLR);
    draw_line(CENTER_X, INTER_B, CENTER_X, WIN_H,  t, CTR_CLR);
    // Between westbound and eastbound halves of horizontal road
    draw_line(0.0,    CENTER_Y, INTER_L, CENTER_Y, t, CTR_CLR);
    draw_line(INTER_R, CENTER_Y, WIN_W,  CENTER_Y, t, CTR_CLR);
}

// ── Stop lines at intersection entries ───────────────────────────────────────
fn draw_stop_lines() {
    let t = 4.0;
    // Southbound  — enters at INTER_T, occupies x: INTER_L..CENTER_X
    draw_line(INTER_L,  INTER_T, CENTER_X, INTER_T, t, STOP_CLR);
    // Northbound  — enters at INTER_B, occupies x: CENTER_X..INTER_R
    draw_line(CENTER_X, INTER_B, INTER_R,  INTER_B, t, STOP_CLR);
    // Westbound   — enters at INTER_R, occupies y: INTER_T..CENTER_Y
    draw_line(INTER_R,  INTER_T, INTER_R,  CENTER_Y, t, STOP_CLR);
    // Eastbound   — enters at INTER_L, occupies y: CENTER_Y..INTER_B
    draw_line(INTER_L,  CENTER_Y, INTER_L, INTER_B,  t, STOP_CLR);
}

// ── Lane direction markers ────────────────────────────────────────────────────
fn draw_lane_markers() {
    let mid_north = INTER_T / 2.0;                       // 140
    let mid_south = INTER_B + (WIN_H - INTER_B) / 2.0;  // 660
    let mid_west  = INTER_L / 2.0;                       // 140
    let mid_east  = INTER_R + (WIN_W - INTER_R) / 2.0;  // 660

    // North arm — southbound (approach direction = PI/2 = ↓)
    lane_mark(SB_R_X, mid_north, PI / 2.0, "r");
    lane_mark(SB_S_X, mid_north, PI / 2.0, "s");
    lane_mark(SB_L_X, mid_north, PI / 2.0, "l");

    // South arm — northbound (approach direction = -PI/2 = ↑)
    lane_mark(NB_L_X, mid_south, -PI / 2.0, "l");
    lane_mark(NB_S_X, mid_south, -PI / 2.0, "s");
    lane_mark(NB_R_X, mid_south, -PI / 2.0, "r");

    // West arm — eastbound (approach direction = 0 = →)
    lane_mark(mid_west, WB_R_Y, 0.0, "r");
    lane_mark(mid_west, WB_S_Y, 0.0, "s");
    lane_mark(mid_west, WB_L_Y, 0.0, "l");

    // East arm — westbound (approach direction = PI = ←)
    lane_mark(mid_east, EB_L_Y, PI, "l");
    lane_mark(mid_east, EB_S_Y, PI, "s");
    lane_mark(mid_east, EB_R_Y, PI, "r");
}

// Draw one direction arrow + route letter for a lane
fn lane_mark(cx: f32, cy: f32, angle: f32, label: &str) {
    dir_arrow(cx, cy, angle, 26.0);
    // Place label to the right of travel direction (perpendicular offset)
    let right = Vec2::new(angle.sin(), -angle.cos()) * 14.0;
    draw_text(label, cx + right.x - 5.0, cy + right.y + 5.0, 16.0, MARK_CLR);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn dir_arrow(cx: f32, cy: f32, angle: f32, len: f32) {
    let dir = Vec2::new(angle.cos(), angle.sin());
    let from = Vec2::new(cx, cy) - dir * (len / 2.0);
    let to   = Vec2::new(cx, cy) + dir * (len / 2.0);

    draw_line(from.x, from.y, to.x, to.y, 2.0, MARK_CLR);

    // Arrowhead triangle
    let perp = Vec2::new(-dir.y, dir.x);
    let head_base = to - dir * 9.0;
    draw_triangle(to, head_base + perp * 5.0, head_base - perp * 5.0, MARK_CLR);
}

fn dash_v(x: f32, y0: f32, y1: f32) {
    let (dash, gap) = (18.0_f32, 10.0_f32);
    let mut y = y0;
    while y < y1 {
        draw_line(x, y, x, (y + dash).min(y1), 1.5, DIV_CLR);
        y += dash + gap;
    }
}

fn dash_h(x0: f32, x1: f32, y: f32) {
    let (dash, gap) = (18.0_f32, 10.0_f32);
    let mut x = x0;
    while x < x1 {
        draw_line(x, y, (x + dash).min(x1), y, 1.5, DIV_CLR);
        x += dash + gap;
    }
}
