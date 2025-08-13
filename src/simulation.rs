use crate::roads::Road;
use crate::traffic_lights::TrafficLight;
use crate::vehicles::Vehicle;

pub struct Simulation {
    pub roads: Vec<Road>,
    pub traffic_lights: Vec<TrafficLight>,
    pub vehicles: Vec<Vehicle>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            roads: Vec::new(),
            traffic_lights: Vec::new(),
            vehicles: Vec::new(),
        }
    }
}
