use bevy::prelude::*;

pub struct ArmorPlugin;
impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Armor>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Armor {}
