// ========================================
// TRAFFIC LIGHT MANAGEMENT
// ========================================
// This file handles all traffic light logic and timing

use macroquad::prelude::*;
use crate::types::*;

// Generate a random turn type for new cars
// Returns: 0 = right turn, 1 = left turn, 2 = go forward
pub fn get_random_turn() -> i32 {
    let random_value = rand::rand() % 3;  // Get 0, 1, or 2
    return random_value as i32;
}

// Calculate how long a green light should last based on nearby cars
pub fn calculate_green_time(cars: &Vec<Car>, lights: &Vec<TrafficLight>, green_index: usize) -> f64 {
    let green_light = &lights[green_index];
    let mut count = 0;  // Count cars that would use this green light

    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    for car in cars {
        if car.direction_x == green_light.direction_x && car.direction_y == green_light.direction_y {
            let mut is_near = false;

            if car.direction_x == 0.0 && car.direction_y == 5.0 {
                if (center_y - car.y) < 200.0 {
                    is_near = true;
                }
            } else if car.direction_x == 0.0 && car.direction_y == -5.0 {
                if (car.y - center_y) < 200.0 {
                    is_near = true;
                }
            } else if car.direction_x == 5.0 && car.direction_y == 0.0 {
                if (center_x - car.x) < 200.0 {
                    is_near = true;
                }
            } else if car.direction_x == -5.0 && car.direction_y == 0.0 {
                if (car.x - center_x) < 200.0 {
                    is_near = true;
                }
            }

            if is_near {
                count += 1;
            }
        }
    }

    let duration = (count as f64) * MIN_GREEN_DURATION;
    if duration > MAX_GREEN_DURATION {
        return MAX_GREEN_DURATION;
    }
    // Ensure minimum green time even when no cars are present
    if duration < MIN_GREEN_DURATION {
        return MIN_GREEN_DURATION;
    }
    return duration;
}

// Update the traffic light system (handles switching between green and red phases)
pub fn update_traffic_lights(lights: &mut Vec<TrafficLight>, cars: &Vec<Car>, traffic_system: &mut TrafficSystem, light_order: &[usize; 4]) {
    let current_time = get_time();

    if traffic_system.is_green_phase {
        let green_index = light_order[traffic_system.current_green_light];
        let green_duration = calculate_green_time(cars, lights, green_index);

        if current_time - traffic_system.last_switch_time >= green_duration {
            traffic_system.is_green_phase = false;
            traffic_system.last_switch_time = current_time;
            
            for light in lights {
                light.is_green = false;
            }
        }
    } else {
        if current_time - traffic_system.last_switch_time >= ALL_RED_DURATION {
            traffic_system.current_green_light = (traffic_system.current_green_light + 1) % light_order.len();
            let green_index = light_order[traffic_system.current_green_light];
            traffic_system.is_green_phase = true;
            traffic_system.last_switch_time = current_time;

            for i in 0..lights.len() {
                if i == green_index {
                    lights[i].is_green = true;
                } else {
                    lights[i].is_green = false;
                }
            }
        }
    }
}
