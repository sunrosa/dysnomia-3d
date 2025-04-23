use bevy::prelude::*;

pub struct SkinPlugin;
impl Plugin for SkinPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Skin>();
    }
}

/// Paint, stealth coatings, etc.
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Skin {}
