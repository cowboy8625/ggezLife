#[derive(Debug, Clone)]
pub enum GameEvent {
    Quit,
    SpeedUp,
    SpeedDown,
    SpawnRandom,
    ClearBoard,
    StartStop,
    ScreenUp,
    ScreenDown,
    ScreenLeft,
    ScreenRight,
    ZoomOut,
    ZoomIn,
    SwapBlock,
}
