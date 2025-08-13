pub enum LightState {
    Red,
    Green,
}

pub struct TrafficLight {
    pub state: LightState,
    pub timer: u32,
    pub position: (i32, i32),
}

impl TrafficLight {
    pub fn new(position: (i32, i32)) -> Self {
        Self {
            state: LightState::Red,
            timer: 0,
            position,
        }
    }
}
