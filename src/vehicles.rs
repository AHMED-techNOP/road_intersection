pub enum Route {
    Left,
    Right,
    Straight,
}

pub struct Vehicle {
    pub position: (f32, f32),
    pub velocity: f32,
    pub route: Route,
    pub color: (u8, u8, u8),
}

impl Vehicle {
    pub fn new(position: (f32, f32), velocity: f32, route: Route, color: (u8, u8, u8)) -> Self {
        Self { position, velocity, route, color }
    }
}
