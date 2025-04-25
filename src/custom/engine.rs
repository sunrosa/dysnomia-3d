use bevy::prelude::*;

pub struct EnginePlugin;
impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Engine>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Engine {}
