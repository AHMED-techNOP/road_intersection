mod roads;
mod traffic_lights;
mod vehicles;
mod simulation;
mod input;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn draw_roads(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // Road dimensions
    let road_width = 100;
    let window_width = 800;
    let window_height = 600;

    // Draw horizontal road
    canvas.set_draw_color(Color::RGB(60, 60, 60));
    let _ = canvas.fill_rect(sdl2::rect::Rect::new(
        0,
        (window_height / 2) - (road_width / 2),
        window_width as u32,
        road_width as u32,
    ));

    // Draw vertical road
    let _ = canvas.fill_rect(sdl2::rect::Rect::new(
        (window_width / 2) - (road_width / 2),
        0,
        road_width as u32,
        window_height as u32,
    ));
}

fn draw_traffic_lights(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    use sdl2::rect::Rect;
    let light_radius = 15;
    let road_width = 100;
    let window_width = 800;
    let window_height = 600;
    let center_x = window_width / 2;
    let center_y = window_height / 2;

    // North (entering from top, flush with intersection)
    let north = (
        center_x - (road_width / 2) - (light_radius * 2),
        center_y - (road_width / 2) - (light_radius * 2),
    );
    // South (entering from bottom, flush with intersection)
    let south = (
        center_x + (road_width / 2) ,
        center_y + (road_width / 2),
    );
    // West (entering from left, flush with intersection)
    let west = (
        center_x - (road_width / 2) - (light_radius * 2),
        center_y + (road_width / 2) ,
    );
    // East (entering from right, flush with intersection)
    let east = (
        center_x + (road_width / 2),
        center_y - (road_width / 2) - (light_radius * 2),
    );

    let lights = [north, south, west, east];
    for &(x, y) in &lights {
        canvas.set_draw_color(Color::RGB(200, 0, 0)); // Red border only
        let _ = canvas.draw_rect(Rect::new(x, y, (light_radius * 2) as u32, (light_radius * 2) as u32));
    }
}

struct Vehicle {
    position: (f32, f32),
    direction: (f32, f32),
    color: Color,
}

fn spawn_vehicle(key: Keycode) -> Option<Vehicle> {
    let road_width = 100.0;
    let window_width = 800.0;
    let window_height = 600.0;
    let center_x = window_width / 2.0;
    let center_y = window_height / 2.0;
    let vehicle_size = 20.0;
    match key {
        Keycode::Up => Some(Vehicle {
            position: (center_x - road_width / 4.0 - vehicle_size / 2.0, window_height - vehicle_size),
            direction: (0.0, -1.0),
            color: Color::RGB(0, 200, 0),
        }),
        Keycode::Down => Some(Vehicle {
            position: (center_x + road_width / 4.0 - vehicle_size / 2.0, 0.0),
            direction: (0.0, 1.0),
            color: Color::RGB(0, 0, 200),
        }),
        Keycode::Left => Some(Vehicle {
            position: (window_width - vehicle_size, center_y + road_width / 4.0 - vehicle_size / 2.0),
            direction: (-1.0, 0.0),
            color: Color::RGB(200, 200, 0),
        }),
        Keycode::Right => Some(Vehicle {
            position: (0.0, center_y - road_width / 4.0 - vehicle_size / 2.0),
            direction: (1.0, 0.0),
            color: Color::RGB(200, 0, 200),
        }),
        _ => None,
    }
}

fn draw_vehicles(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, vehicles: &Vec<Vehicle>) {
    use sdl2::rect::Rect;
    let vehicle_size = 20;
    for v in vehicles {
        canvas.set_draw_color(v.color);
        let _ = canvas.fill_rect(Rect::new(
            v.position.0 as i32,
            v.position.1 as i32,
            vehicle_size,
            vehicle_size,
        ));
    }
}

fn update_vehicles(vehicles: &mut Vec<Vehicle>) {
    let speed = 2.0;
    let window_width = 800.0;
    let window_height = 600.0;
    let vehicle_size = 20.0;
    for v in vehicles.iter_mut() {
        v.position.0 += v.direction.0 * speed;
        v.position.1 += v.direction.1 * speed;
    }
    // Remove vehicles that are off-screen
    vehicles.retain(|v| {
        v.position.0 + vehicle_size > 0.0 && v.position.0 < window_width &&
        v.position.1 + vehicle_size > 0.0 && v.position.1 < window_height
    });
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection Simulation", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut vehicles: Vec<Vehicle> = Vec::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(k), repeat: false, .. } => {
                    if let Some(vehicle) = spawn_vehicle(k) {
                        vehicles.push(vehicle);
                    }
                }
                _ => {}
            }
        }
        update_vehicles(&mut vehicles);
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        draw_roads(&mut canvas);
        draw_traffic_lights(&mut canvas);
        draw_vehicles(&mut canvas, &vehicles);
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}
