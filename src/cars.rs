// ========================================
// CAR SPAWNING AND MOVEMENT
// ========================================
// This file handles creating new cars and managing their movement

use macroquad::prelude::*;
use crate::types::*;
use crate::traffic::get_random_turn;

// ========================================
// CAR SPAWNING FUNCTIONS
// ========================================

// Try to spawn a car from the top (moving down)
pub fn try_spawn_car_0(x: f32, y: f32, last_spawn_time: &mut f32, cars: &mut Vec<Car>) {
    let current_time = get_time() as f32;
    if current_time - *last_spawn_time < MIN_SPAWN_DELAY {
        return;
    }

    let new_car = Car {
        x: x,
        y: y,
        direction_x: 0.0,
        direction_y: 5.0,
        turn_type: get_random_turn(),
        has_turned: false,
    };

    if !is_car_too_close(&new_car, cars) {
        cars.push(new_car);
    }

    *last_spawn_time = current_time;
}

// Try to spawn a car from the bottom (moving up)
pub fn try_spawn_car_1(x: f32, y: f32, last_spawn_time: &mut f32, cars: &mut Vec<Car>) {
    let current_time = get_time() as f32;
    if current_time - *last_spawn_time < MIN_SPAWN_DELAY {
        return;
    }

    let new_car = Car {
        x: x,
        y: y,
        direction_x: 0.0,
        direction_y: -5.0,
        turn_type: get_random_turn(),
        has_turned: false,
    };

    if !is_car_too_close(&new_car, cars) {
        cars.push(new_car);
    }

    *last_spawn_time = current_time;
}

// Try to spawn a car from the left (moving right)
pub fn try_spawn_car_2(x: f32, y: f32, last_spawn_time: &mut f32, cars: &mut Vec<Car>) {
    let current_time = get_time() as f32;
    if current_time - *last_spawn_time < MIN_SPAWN_DELAY {
        return;
    }

    let new_car = Car {
        x: x,
        y: y,
        direction_x: 5.0,
        direction_y: 0.0,
        turn_type: get_random_turn(),
        has_turned: false,
    };

    if !is_car_too_close(&new_car, cars) {
        cars.push(new_car);
    }

    *last_spawn_time = current_time;
}

// Try to spawn a car from the right (moving left)
pub fn try_spawn_car_3(x: f32, y: f32, last_spawn_time: &mut f32, cars: &mut Vec<Car>) {
    let current_time = get_time() as f32;
    if current_time - *last_spawn_time < MIN_SPAWN_DELAY {
        return;
    }

    let new_car = Car {
        x: x,
        y: y,
        direction_x: -5.0,
        direction_y: 0.0,
        turn_type: get_random_turn(),
        has_turned: false,
    };

    if !is_car_too_close(&new_car, cars) {
        cars.push(new_car);
    }

    *last_spawn_time = current_time;
}

// ========================================
// CAR MOVEMENT AND BEHAVIOR
// ========================================

// Check if a car should stop at a traffic light
pub fn should_stop_for_light(car: &Car, lights: &Vec<TrafficLight>) -> bool {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    
    let mut relevant_light: Option<&TrafficLight> = None;
    for light in lights {
        if light.direction_x == car.direction_x && light.direction_y == car.direction_y {
            relevant_light = Some(light);
            break;
        }
    }

    if let Some(light) = relevant_light {
        if light.is_green {
            return false;
        }
        
        if car.direction_x == 0.0 && car.direction_y == 5.0 {
            return car.y + 2.0 * INTERSECTION_SIZE == center_y;
        } else if car.direction_x == 0.0 && car.direction_y == -5.0 {
            return car.y - INTERSECTION_SIZE == center_y;
        } else if car.direction_x == 5.0 && car.direction_y == 0.0 {
            return car.x + 2.0 * INTERSECTION_SIZE == center_x;
        } else if car.direction_x == -5.0 && car.direction_y == 0.0 {
            return car.x - INTERSECTION_SIZE == center_x;
        }
    }
    
    return false;
}

// Try to make a car turn at the intersection
pub fn try_to_turn(car: &mut Car) {
    if car.turn_type == 2 || car.has_turned {
        return;
    }
    
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    let mut can_turn = false;

    if car.direction_x == 5.0 && car.direction_y == 0.0 && car.turn_type == 0 {
        can_turn = (car.x == center_x - INTERSECTION_SIZE) && (car.y == center_y);
    } else if car.direction_x == 0.0 && car.direction_y == 5.0 && car.turn_type == 1 {
        can_turn = (car.x == center_x - INTERSECTION_SIZE) && (car.y == center_y);
    } else if car.direction_x == -5.0 && car.direction_y == 0.0 && car.turn_type == 1 {
        can_turn = (car.x == center_x - INTERSECTION_SIZE) && (car.y == center_y - INTERSECTION_SIZE);
    } else if car.direction_x == 0.0 && car.direction_y == 5.0 && car.turn_type == 0 {
        can_turn = (car.x == center_x - INTERSECTION_SIZE) && (car.y == center_y - INTERSECTION_SIZE);
    } else if car.direction_x == -5.0 && car.direction_y == 0.0 && car.turn_type == 0 {
        can_turn = (car.x == center_x) && (car.y == center_y - INTERSECTION_SIZE);
    } else if car.direction_x == 0.0 && car.direction_y == -5.0 && car.turn_type == 1 {
        can_turn = (car.x == center_x) && (car.y == center_y - INTERSECTION_SIZE);
    } else if car.direction_x == 0.0 && car.direction_y == -5.0 && car.turn_type == 0 {
        can_turn = (car.x == center_x) && (car.y == center_y);
    } else if car.direction_x == 5.0 && car.direction_y == 0.0 && car.turn_type == 1 {
        can_turn = (car.x == center_x) && (car.y == center_y);
    }

    if can_turn {
        if car.turn_type == 1 {
            let old_dir_x = car.direction_x;
            car.direction_x = car.direction_y;
            car.direction_y = -old_dir_x;
        } else if car.turn_type == 0 {
            let old_dir_x = car.direction_x;
            car.direction_x = -car.direction_y;
            car.direction_y = old_dir_x;
        }
        car.has_turned = true;
    }
}

// Check if a new car would be too close to existing cars
pub fn is_car_too_close(car: &Car, other_cars: &Vec<Car>) -> bool {
    for other_car in other_cars {
        if other_car.x == car.x && other_car.y == car.y {
            continue;
        }
        if other_car.direction_x != car.direction_x || other_car.direction_y != car.direction_y {
            continue;
        }

        let distance_x = other_car.x - car.x;
        let distance_y = other_car.y - car.y;

        if car.direction_x == 0.0 && car.direction_y == 5.0 {
            if distance_y > 0.0 && distance_x.abs() < INTERSECTION_SIZE && distance_y < MIN_GAP {
                return true;
            }
        } else if car.direction_x == 0.0 && car.direction_y == -5.0 {
            if distance_y < 0.0 && distance_x.abs() < INTERSECTION_SIZE && -distance_y < MIN_GAP {
                return true;
            }
        } else if car.direction_x == 5.0 && car.direction_y == 0.0 {
            if distance_x > 0.0 && distance_y.abs() < INTERSECTION_SIZE && distance_x < MIN_GAP {
                return true;
            }
        } else if car.direction_x == -5.0 && car.direction_y == 0.0 {
            if distance_x < 0.0 && distance_y.abs() < INTERSECTION_SIZE && -distance_x < MIN_GAP {
                return true;
            }
        }
    }
    return false;
}
