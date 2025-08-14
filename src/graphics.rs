// ========================================
// RENDERING AND GRAPHICS
// ========================================
// This file handles all the visual drawing functions

use macroquad::prelude::*;
use crate::types::*;
use crate::cars::{should_stop_for_light, is_car_too_close, try_to_turn};



// ========================================
// CAR DRAWING AND MOVEMENT
// ========================================

// Update car positions and draw them on screen
pub fn update_and_draw_cars(cars: &mut Vec<Car>, lights: &Vec<TrafficLight>) {
    let cars_copy = cars.clone();
    
    for car in cars {
        // Choose car colors based on turn type
        let car_color = match car.turn_type {
            0 => Color::new(0.8, 0.2, 0.8, 1.0), // Purple for right turn
            1 => Color::new(0.9, 0.8, 0.2, 1.0), // Golden yellow for left turn  
            _ => Color::new(0.2, 0.4, 0.9, 1.0), // Blue for forward
        };

        // Draw car body
        draw_rectangle(car.x, car.y, INTERSECTION_SIZE, INTERSECTION_SIZE, car_color);
        
        // Draw car outline
        draw_rectangle_lines(car.x, car.y, INTERSECTION_SIZE, INTERSECTION_SIZE, 2.0, BLACK);
        
        // Draw direction indicator (small arrow)
        let arrow_color = WHITE;
        let center_x = car.x + INTERSECTION_SIZE / 2.0;
        let center_y = car.y + INTERSECTION_SIZE / 2.0;
        let arrow_size = 6.0;
        
        if car.direction_x > 0.0 { // Moving right
            draw_triangle(
                vec2(center_x - arrow_size, center_y - arrow_size),
                vec2(center_x - arrow_size, center_y + arrow_size), 
                vec2(center_x + arrow_size, center_y),
                arrow_color
            );
        } else if car.direction_x < 0.0 { // Moving left
            draw_triangle(
                vec2(center_x + arrow_size, center_y - arrow_size),
                vec2(center_x + arrow_size, center_y + arrow_size),
                vec2(center_x - arrow_size, center_y),
                arrow_color
            );
        } else if car.direction_y > 0.0 { // Moving down
            draw_triangle(
                vec2(center_x - arrow_size, center_y - arrow_size),
                vec2(center_x + arrow_size, center_y - arrow_size),
                vec2(center_x, center_y + arrow_size),
                arrow_color
            );
        } else if car.direction_y < 0.0 { // Moving up
            draw_triangle(
                vec2(center_x - arrow_size, center_y + arrow_size),
                vec2(center_x + arrow_size, center_y + arrow_size),
                vec2(center_x, center_y - arrow_size),
                arrow_color
            );
        }

        // Move the car if it's not blocked
        if should_stop_for_light(car, lights) || is_car_too_close(car, &cars_copy) {
            continue;
        }

        car.x += car.direction_x;
        car.y += car.direction_y;
        try_to_turn(car);
    }
}

// ========================================
// TRAFFIC LIGHT DRAWING
// ========================================

// Draw all traffic lights with realistic appearance
pub fn draw_lights(lights: &Vec<TrafficLight>) {
    for light in lights {
        let light_size = INTERSECTION_SIZE * 1.2;
        
        // Draw traffic light box (dark gray)
        let box_color = Color::new(0.2, 0.2, 0.2, 1.0);
        draw_rectangle(light.x - 3.0, light.y - 3.0, light_size + 6.0, light_size + 6.0, box_color);
        
        // Draw traffic light background (lighter gray)
        let bg_color = Color::new(0.4, 0.4, 0.4, 1.0);
        draw_rectangle(light.x, light.y, light_size, light_size, bg_color);
        
        // Calculate light positions
        let center_x = light.x + light_size / 2.0;
        let center_y = light.y + light_size / 2.0;
        let light_radius = light_size / 6.0;
        
        // Draw red light (top)
        let red_color = if light.is_green { 
            Color::new(0.3, 0.1, 0.1, 1.0) // Dim red
        } else { 
            Color::new(1.0, 0.1, 0.1, 1.0) // Bright red
        };
        draw_circle(center_x, center_y - light_radius, light_radius, red_color);
        
        // Draw green light (bottom)
        let green_color = if light.is_green {
            Color::new(0.1, 0.8, 0.1, 1.0) // Bright green  
        } else {
            Color::new(0.1, 0.3, 0.1, 1.0) // Dim green
        };
        draw_circle(center_x, center_y + light_radius, light_radius, green_color);
        
        // Draw light outlines
        draw_circle_lines(center_x, center_y - light_radius, light_radius, 2.0, BLACK);
        draw_circle_lines(center_x, center_y + light_radius, light_radius, 2.0, BLACK);
        
        // Draw outer box outline
        draw_rectangle_lines(light.x - 3.0, light.y - 3.0, light_size + 6.0, light_size + 6.0, 2.0, BLACK);
    }
}

// ========================================
// ROAD DRAWING
// ========================================

// Draw roads with realistic markings and colors
pub fn draw_roads(center_x: f32, center_y: f32) {
    // Draw background
    clear_background(Color::new(0.1, 0.4, 0.1, 1.0)); // Dark green grass
    
    // Draw road surfaces (gray asphalt)
    let road_color = Color::new(0.3, 0.3, 0.3, 1.0);
    
    // Vertical road
    draw_rectangle(
        center_x - ROAD_WIDTH / 2.0, 
        0.0, 
        ROAD_WIDTH, 
        screen_height(), 
        road_color
    );
    
    // Horizontal road  
    draw_rectangle(
        0.0, 
        center_y - ROAD_WIDTH / 2.0, 
        screen_width(), 
        ROAD_WIDTH, 
        road_color
    );
    
    // Draw intersection area (slightly darker)
    let intersection_color = Color::new(0.25, 0.25, 0.25, 1.0);
    draw_rectangle(
        center_x - ROAD_WIDTH / 2.0,
        center_y - ROAD_WIDTH / 2.0,
        ROAD_WIDTH,
        ROAD_WIDTH,
        intersection_color
    );

    // Draw road markings (white lines)
    let line_color = Color::new(0.9, 0.9, 0.9, 1.0);
    let line_width = 2.0;
    
    // Vertical road edges
    draw_line(center_x - ROAD_WIDTH / 2.0, 0.0, center_x - ROAD_WIDTH / 2.0, screen_height(), line_width, line_color);
    draw_line(center_x + ROAD_WIDTH / 2.0, 0.0, center_x + ROAD_WIDTH / 2.0, screen_height(), line_width, line_color);
    
    // Horizontal road edges
    draw_line(0.0, center_y - ROAD_WIDTH / 2.0, screen_width(), center_y - ROAD_WIDTH / 2.0, line_width, line_color);
    draw_line(0.0, center_y + ROAD_WIDTH / 2.0, screen_width(), center_y + ROAD_WIDTH / 2.0, line_width, line_color);
    
    // Center lane dividers (dashed lines)
    draw_dashed_line(center_x, 0.0, center_x, center_y - ROAD_WIDTH / 2.0, line_width, YELLOW);
    draw_dashed_line(center_x, center_y + ROAD_WIDTH / 2.0, center_x, screen_height(), line_width, YELLOW);
    draw_dashed_line(0.0, center_y, center_x - ROAD_WIDTH / 2.0, center_y, line_width, YELLOW);
    draw_dashed_line(center_x + ROAD_WIDTH / 2.0, center_y, screen_width(), center_y, line_width, YELLOW);

    // Draw sidewalks
    let sidewalk_color = Color::new(0.7, 0.7, 0.7, 1.0);
    let sidewalk_width = 15.0;
    
    // Vertical sidewalks
    draw_rectangle(center_x - ROAD_WIDTH / 2.0 - sidewalk_width, 0.0, sidewalk_width, screen_height(), sidewalk_color);
    draw_rectangle(center_x + ROAD_WIDTH / 2.0, 0.0, sidewalk_width, screen_height(), sidewalk_color);
    
    // Horizontal sidewalks
    draw_rectangle(0.0, center_y - ROAD_WIDTH / 2.0 - sidewalk_width, screen_width(), sidewalk_width, sidewalk_color);
    draw_rectangle(0.0, center_y + ROAD_WIDTH / 2.0, screen_width(), sidewalk_width, sidewalk_color);
}

// Draw dashed lines for road markings
pub fn draw_dashed_line(start_x: f32, start_y: f32, end_x: f32, end_y: f32, thickness: f32, color: Color) {
    let total_length = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
    let dash_length = 15.0;
    let gap_length = 10.0;
    let segment_length = dash_length + gap_length;
    
    let num_segments = (total_length / segment_length) as i32;
    let dx = (end_x - start_x) / total_length;
    let dy = (end_y - start_y) / total_length;
    
    for i in 0..num_segments {
        let dash_start_x = start_x + dx * (i as f32 * segment_length);
        let dash_start_y = start_y + dy * (i as f32 * segment_length);
        let dash_end_x = start_x + dx * (i as f32 * segment_length + dash_length);
        let dash_end_y = start_y + dy * (i as f32 * segment_length + dash_length);
        
        draw_line(dash_start_x, dash_start_y, dash_end_x, dash_end_y, thickness, color);
    }
}
