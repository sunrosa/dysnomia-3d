use bevy::prelude::*;

use super::material::Material;

/// The tolerance for how far from `1.0` the armor layers are allowed to stray from.
const ARMOR_LAYER_TOLERANCE: f64 = 0.000001;

pub struct ArmorPlugin;
impl Plugin for ArmorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ArmorLayers>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ArmorLayers {
    /// Layers of armor, from inside to out
    /// - `0`: Ratio of how much of the armor the layer takes up, adding up to exactly `1.0` across all elements.
    /// - `1`: The material of the layer
    layers: Vec<(f64, Material)>,
}

impl ArmorLayers {
    /// - `true`: The armor layers are valid.
    /// - `false`: The armor layers are invalid.
    fn verify(&self) -> bool {
        let sum = self.layers.iter().fold(0., |acc: f64, layer| acc + layer.0);

        (1. - ARMOR_LAYER_TOLERANCE..=1. + ARMOR_LAYER_TOLERANCE).contains(&sum)
    }
}
