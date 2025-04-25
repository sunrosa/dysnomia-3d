use bevy::prelude::*;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Material>()
            .register_type::<ExplosiveMaterial>();
    }
}

/// MANY of these fields are unneeded and will need to be purged
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Material {
    name: String,
    ultimate_tensile_strength: f64,
    yield_strength: f64,
    compressive_strength: f64,
    shear_strength: f64,
    fracture_strength: f64,
    spall_strength: f64,

    youngs_modulus: f64,
    shear_modulus: f64,
    poissons_ratio: f64,
    bulk_modulus: f64,

    elongation_at_break: f64,
    reduction_in_area: f64,
    fracture_toughness: f64,
    impact_toughness: f64,
    modulus_of_toughness: f64,

    vickers_hardness: f64,

    /// This should probably be a thermal curve
    density: f64,

    specific_heat_capacity: f64,
    thermal_conductivity: f64,
    coefficient_of_thermal_expansion: f64,
    melting_point: f64,
    glass_transition_temperature: f64,

    strain_rate_sensitivity_parameter: f64,
    thermal_softening_exponent: f64,
    reference_strain_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ExplosiveMaterial {}
