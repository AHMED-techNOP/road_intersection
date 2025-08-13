use sdl2::keyboard::Keycode;

pub enum InputEvent {
    SpawnUp,
    SpawnDown,
    SpawnLeft,
    SpawnRight,
    SpawnRandom,
    Quit,
    None,
}

pub fn map_keycode(key: Option<Keycode>) -> InputEvent {
    match key {
        Some(Keycode::Up) => InputEvent::SpawnUp,
        Some(Keycode::Down) => InputEvent::SpawnDown,
        Some(Keycode::Left) => InputEvent::SpawnLeft,
        Some(Keycode::Right) => InputEvent::SpawnRight,
        Some(Keycode::R) => InputEvent::SpawnRandom,
        Some(Keycode::Escape) => InputEvent::Quit,
        _ => InputEvent::None,
    }
}
