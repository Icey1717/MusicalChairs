pub const CAMERA_FAR: f32 = 1000.0 - 1.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
    //Paused,
}