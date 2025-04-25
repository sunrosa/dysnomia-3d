use bevy::prelude::*;

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ship>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Ship {}
