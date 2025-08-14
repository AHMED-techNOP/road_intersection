// ========================================
// DATA TYPES AND STRUCTURES
// ========================================
// This file contains all the main data structures used in the simulation

use macroquad::prelude::*;

// ========================================
// GAME SETTINGS AND CONSTANTS
// ========================================

pub const ROAD_WIDTH: f32 = 80.0;           
pub const INTERSECTION_SIZE: f32 = 25.0;    
pub const MIN_SPAWN_DELAY: f32 = 0.2;       
pub const MIN_GAP: f32 = 65.0;             
pub const MIN_GREEN_DURATION: f64 = 4.0;    
pub const MAX_GREEN_DURATION: f64 = 12.0;    
pub const ALL_RED_DURATION: f64 = 1.0;      

// ========================================
// DATA STRUCTURES
// ========================================


pub struct TrafficLight {
    pub x: f32,              
    pub y: f32,             
    pub is_green: bool,      
    pub direction_x: f32,    
    pub direction_y: f32,   
}

#[derive(Clone)]
pub struct Car {
    pub x: f32,              
    pub y: f32,              
    pub direction_x: f32,    
    pub direction_y: f32,    
    pub turn_type: i32,      
    pub has_turned: bool,    
}

pub struct TrafficSystem {
    pub current_green_light: usize,    
    pub is_green_phase: bool,          
    pub last_switch_time: f64,         
}
