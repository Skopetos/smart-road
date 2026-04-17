# Smart Intersection Management — Concept

## Core Idea: Reservation-Based Protocol

Instead of traffic lights, we use an **AIM (Autonomous Intersection Management)** protocol. Each vehicle "requests" a time-slot reservation for a set of tiles it will occupy as it crosses the intersection. The intersection manager grants or denies based on conflict detection.

Think of it like booking a table at a restaurant — the intersection is the restaurant, tiles are seats, and time slots are booking windows.

---

## The Intersection Grid

The intersection area is subdivided into a fine grid of **tiles** (e.g., 10×10 or 20×20 cells). Each route (right/straight/left from each direction) traces a specific **path** through these tiles.

```
┌──────────────────────────┐
│   [tile][tile][tile]...  │
│   [tile][tile][tile]...  │
│         ...              │
└──────────────────────────┘
```

- A straight path from north→south occupies a column of tiles.
- A right turn traces a small arc (few tiles).
- A left turn traces a longer arc (more tiles, more conflict potential).

---

## Reservation Table

The manager maintains a `HashMap<(tile_x, tile_y, time_slot), vehicle_id>`. When a vehicle approaches:

1. It sends a **request** with its: current speed, route, and ETA to the intersection entry.
2. The manager simulates the vehicle's path tile by tile, computing which tiles it would occupy at which time slots.
3. If **no conflicts** → reservation granted, vehicle proceeds.
4. If **conflict** → vehicle is told to slow down or stop, then re-requests when its new ETA might find a free slot.

This avoids stop-and-go traffic lights and allows multiple vehicles to cross simultaneously as long as their paths don't overlap in time.

---

## Vehicle Physics

Each AV has:
- **position** (x, y) as f64
- **velocity** — at least 3 discrete levels: `Slow=50`, `Normal=100`, `Fast=150` px/s
- **acceleration/deceleration** (bonus: smooth ramping between speeds)
- **route** — Right / Straight / Left → determines tile path
- **state** — `Approaching`, `Reserved`, `InIntersection`, `Exiting`

Safety distance is enforced by checking the distance to the vehicle ahead on the same lane and capping speed accordingly.

---

## Simultaneous Non-Conflicting Paths

Many paths can coexist without collision:

| Route A          | Route B          | Conflict? |
|------------------|------------------|-----------|
| North → Right    | South → Right    | No        |
| North → Straight | East → Right     | No        |
| North → Left     | South → Left     | Yes       |
| North → Straight | South → Straight | Yes       |

The tile reservation system captures this automatically — no hand-coded rules needed.

---

## Animation Strategy

- Cars are **sprite sheets** (or colored rectangles for MVP) that rotate to face their direction of travel.
- Each route has a **pre-computed waypoint curve** (straight line or Bezier arc for turns).
- At each frame, a vehicle advances along its waypoint curve by `velocity × delta_time`.
- The car's render angle = direction from previous waypoint to current waypoint.

---

## Step-by-Step Plan

| Step | What |
|------|------|
| **1** | Project scaffold — `cargo init`, add `macroquad`, verify window opens |
| **2** | Draw the static intersection (roads, lanes, direction arrows) |
| **3** | Define vehicle struct, routes, waypoints per route |
| **4** | Spawn vehicles on key presses, drive them along waypoints with velocity |
| **5** | Safety distance enforcement between vehicles on the same lane |
| **6** | Intersection tile grid + reservation table |
| **7** | AIM request/grant/deny logic |
| **8** | Multi-vehicle simultaneous crossing |
| **9** | Statistics collection + ESC stats window |
| **10** | Polish: sprite rotation, smooth animation, random vehicle spawning (R key) |
| **Bonus** | Acceleration/deceleration curves |

---

## Rendering Library

**`macroquad`** — built-in game loop, keyboard events, texture drawing, shape drawing, no windowing boilerplate, fast compile times.

---

## Commands

| Key         | Action |
|-------------|--------|
| Arrow Up    | Spawn vehicle from south → north |
| Arrow Down  | Spawn vehicle from north → south |
| Arrow Right | Spawn vehicle from west → east |
| Arrow Left  | Spawn vehicle from east → west |
| R           | Continuously spawn random vehicles |
| Esc         | Stop simulation, show statistics window |

---

## Statistics

Displayed after Esc is pressed:

- Max number of vehicles that passed the intersection
- Max velocity achieved by any vehicle
- Min velocity reached by any vehicle
- Max time any vehicle took to cross the intersection
- Min time any vehicle took to cross the intersection
- Number of close calls (safe distance violated between two vehicles)








 Two vehicles share the exact same path when they have the same origin and route. So we only need to compare within (origin, route) 
  groups.                                                                                                                                         
                                                                                                                                                  
  Algorithm per frame (in main.rs):                                                                                                               
  1. For each vehicle i, scan all other vehicles with same (origin, route)                                                                        
  2. Determine which one is "ahead" using wp_idx (higher = further along path)                                                                    
  3. Compute bumper-to-bumper gap = Euclidean distance between centers - CAR_LENGTH
  4. desired_velocity = SPEED_NORMAL * (gap / SAFE_DIST), clamped to [0, SPEED_NORMAL]                                                            
  5. Set v.velocity, then call v.update()                                             
                                                                                                                                                  
  Two small helper functions in main.rs: path_gap(follower, leader) -> f32 and desired_velocity(gap) -> f32