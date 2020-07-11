use crate::game_event::GameEvent;
use std::collections::HashMap;
use ggez::event::KeyCode;

type KeyMaps = HashMap<KeyCode, GameEvent>;

pub struct Mapper {
    key_maps: KeyMaps
}

impl Mapper {
    pub fn new() -> Self {
        Self { key_maps: HashMap::new() }
    }

    pub fn add_key(&mut self, key: KeyCode, event: GameEvent) {
        self.key_maps.insert(key, event);
    }

    pub fn get(&mut self, key: KeyCode) -> Option<GameEvent> {
        Some(self.key_maps.get(&key)?.clone())
    }
}

pub fn key_mapper() -> Mapper {
    use crate::game_event::GameEvent::*;
    let mut map = Mapper::new();

    map.add_key(KeyCode::Escape, Quit);
    map.add_key(KeyCode::Up, SpeedUp);
    map.add_key(KeyCode::Down, SpeedDown);
    map.add_key(KeyCode::R, SpawnRandom);
    map.add_key(KeyCode::C, ClearBoard);
    map.add_key(KeyCode::Return, StartStop);
    map.add_key(KeyCode::K, ScreenUp);
    map.add_key(KeyCode::J, ScreenDown);
    map.add_key(KeyCode::H, ScreenLeft);
    map.add_key(KeyCode::L, ScreenRight);
    map.add_key(KeyCode::Equals, ZoomOut);
    map.add_key(KeyCode::Minus, ZoomIn);
    map.add_key(KeyCode::Key1, SwapBlock);

    map
}

