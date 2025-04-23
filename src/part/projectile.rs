use bevy::prelude::*;

use super::skin::Skin;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ProjectileType>()
            .register_type::<Fuze>()
            .register_type::<Warhead>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ProjectileType {
    /// Coatings, such as stealth paint
    skin: Option<Skin>,

    /// Warhead trigger fuze
    fuze: Option<Fuze>,

    /// Radius in `m`
    diameter: f64,
    /// Length in `m`
    length: f64,
    /// The material that the projectile/penetrator is made of
    material: Material,

    /// If [`Some`], diameter of the projectile's sabot. If [`None`], the projectile has no sabot.
    sabot_diameter: Option<f64>,

    /// Should the projectile be fin-stabilized? This only matters inside of atmospheres.
    fins: bool,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum Fuze {
    Proximity,
    Contact,
    ContactDelay,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum Warhead {
    DumbKinetic,
    KineticFan,
    Sabot,
    Flechette,
    Shot,
    HE,
    HEAT,
    HEIAP,
    HESH,
}
