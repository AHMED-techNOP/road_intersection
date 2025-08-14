
use macroquad::prelude::*;

// Import our custom modules
mod types;
mod traffic;
mod cars;
mod graphics;

// Import everything we need from our modules
use types::*;
use traffic::*;
use cars::*;
use graphics::*;

// ========================================
// WINDOW SETUP
// ========================================

// Configure the game window
fn window_conf() -> Conf {
    Conf {
        window_title: "Traffic Intersection Simulator".to_string(),
        window_width: 1000,          // Window width in pixels
        window_height: 800,          // Window height in pixels
        window_resizable: false,     // Don't allow window resizing
        fullscreen: false,           // Run in windowed mode (not fullscreen)
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_center_x = screen_width() / 2.0;
    let screen_center_y = screen_height() / 2.0;

    // Car spawn points
    let spawn_point_0_x = screen_center_x - INTERSECTION_SIZE;
    let spawn_point_0_y = 0.0;
    let spawn_point_1_x = screen_center_x;
    let spawn_point_1_y = screen_height() - INTERSECTION_SIZE;
    let spawn_point_2_x = 0.0;
    let spawn_point_2_y = screen_height() / 2.0;
    let spawn_point_3_x = screen_width() - INTERSECTION_SIZE;
    let spawn_point_3_y = screen_height() / 2.0 - INTERSECTION_SIZE;

    // Create traffic lights
    let mut lights: Vec<TrafficLight> = Vec::new();
    
    let light_0 = TrafficLight {
        x: screen_center_x - 2.0 * INTERSECTION_SIZE,
        y: screen_center_y - 2.0 * INTERSECTION_SIZE,
        is_green: false,
        direction_x: 0.0,
        direction_y: 5.0,
    };
    lights.push(light_0);
    
    let light_1 = TrafficLight {
        x: screen_center_x + INTERSECTION_SIZE,
        y: screen_center_y - 2.0 * INTERSECTION_SIZE,
        is_green: false,
        direction_x: -5.0,
        direction_y: 0.0,
    };
    lights.push(light_1);
    
    let light_2 = TrafficLight {
        x: screen_center_x - 2.0 * INTERSECTION_SIZE,
        y: screen_center_y + INTERSECTION_SIZE,
        is_green: false,
        direction_x: 5.0,
        direction_y: 0.0,
    };
    lights.push(light_2);
    
    let light_3 = TrafficLight {
        x: screen_center_x + INTERSECTION_SIZE,
        y: screen_center_y + INTERSECTION_SIZE,
        is_green: false,
        direction_x: 0.0,
        direction_y: -5.0,
    };
    lights.push(light_3);

    let mut cars: Vec<Car> = Vec::new();
    let mut last_spawn_time_0: f32 = 0.0;
    let mut last_spawn_time_1: f32 = 0.0;
    let mut last_spawn_time_2: f32 = 0.0;
    let mut last_spawn_time_3: f32 = 0.0;
    
    let mut traffic_system = TrafficSystem {
        current_green_light: 0,
        is_green_phase: false,
        last_switch_time: 0.0,
    };
    
    // Order in which lights turn green (creates smooth traffic flow)
    let light_order: [usize; 4] = [2, 0, 3, 1];  // Left, Top, Right, Bottom

    loop {
        // Remove cars that are off screen
        let mut i = 0;
        while i < cars.len() {
            let car = &cars[i];
            if car.x < -INTERSECTION_SIZE || car.x > screen_width() + INTERSECTION_SIZE ||
               car.y < -INTERSECTION_SIZE || car.y > screen_height() + INTERSECTION_SIZE {
                cars.remove(i);
            } else {
                i += 1;
            }
        }

        // Update traffic lights
        update_traffic_lights(&mut lights, &cars, &mut traffic_system, &light_order);

        // Draw everything (background is handled in draw_roads)

        if is_key_down(KeyCode::Escape) {
            break;
        }

        // Handle car spawning with keyboard
        if is_key_down(KeyCode::Down) {
            try_spawn_car_0(spawn_point_0_x, spawn_point_0_y, &mut last_spawn_time_0, &mut cars);
        }
        if is_key_down(KeyCode::Up) {
            try_spawn_car_1(spawn_point_1_x, spawn_point_1_y, &mut last_spawn_time_1, &mut cars);
        }
        if is_key_down(KeyCode::Right) {
            try_spawn_car_2(spawn_point_2_x, spawn_point_2_y, &mut last_spawn_time_2, &mut cars);
        }
        if is_key_down(KeyCode::Left) {
            try_spawn_car_3(spawn_point_3_x, spawn_point_3_y, &mut last_spawn_time_3, &mut cars);
        }
        if is_key_down(KeyCode::R) {
            let random_choice = (rand::rand() % 4) as i32;
            if random_choice == 0 {
                try_spawn_car_0(spawn_point_0_x, spawn_point_0_y, &mut last_spawn_time_0, &mut cars);
            } else if random_choice == 1 {
                try_spawn_car_1(spawn_point_1_x, spawn_point_1_y, &mut last_spawn_time_1, &mut cars);
            } else if random_choice == 2 {
                try_spawn_car_2(spawn_point_2_x, spawn_point_2_y, &mut last_spawn_time_2, &mut cars);
            } else {
                try_spawn_car_3(spawn_point_3_x, spawn_point_3_y, &mut last_spawn_time_3, &mut cars);
            }
        }

        // Draw everything in the right order
        draw_roads(screen_center_x, screen_center_y);
        draw_lights(&lights);
        update_and_draw_cars(&mut cars, &lights);

        next_frame().await;
    }
}
