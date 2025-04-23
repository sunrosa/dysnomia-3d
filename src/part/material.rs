use bevy::prelude::*;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Material>()
            .register_type::<ExplosiveMaterial>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Material {}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ExplosiveMaterial {}
