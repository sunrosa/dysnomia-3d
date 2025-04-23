//! Bracketed, expensive simulations could be ran on the very first occurrence of an impact pair between a specific projectile and specific armor, and then every following impact between that same pair could cheaply base itself off the saved results of the expensive simulations.

use bevy::prelude::*;

use super::material::{ExplosiveMaterial, Material};
use super::skin::Skin;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ProjectileType>()
            .register_type::<ExplosiveWarhead>()
            .register_type::<Warhead>()
            .register_type::<Fuze>()
            .register_type::<ShotPellet>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ProjectileType {
    /// Coatings, such as stealth paint
    skin: Option<Skin>,

    /// Radius in `m`
    diameter: f64,
    /// Length in `m`
    length: f64,
    /// The material that the projectile/penetrator is made of
    shell_material: Material,

    /// The explosive warhead the projectile carries. If [`None`], the projectile is kinetic.
    warhead: Warhead,

    /// If [`Some`], diameter of the projectile's sabot. If [`None`], the projectile has no sabot.
    sabot_diameter: Option<f64>,

    /// Should the projectile be fin-stabilized? This only matters inside of atmospheres.
    fins: bool,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ExplosiveWarhead {
    /// Warhead trigger fuze
    pub fuze: Fuze,
    /// Warhead material
    pub warhead_material: ExplosiveMaterial,
    /// Ratio between `0.0` and `1.0` that represents how much of the projectile is shell, and how much is warhead.
    pub warhead_ratio: f32,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum Warhead {
    /// No warhead. The projectile is purely "dumb" kinetic penetrator, such as a bullet.
    None,
    /// Something like [this](https://upload.wikimedia.org/wikipedia/commons/8/8a/CASATCoOrbital01.jpg)
    KineticFan,
    /// Many small projectiles deployed at terminal phase, like a shotgun
    Shot {
        shape: ShotPellet,
        count: u32,
    },
    Explosive(ExplosiveWarhead),
}

/// Types of fuzes used to trigger [`Warheads`](Warhead)
#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum Fuze {
    Proximity,
    Contact,
    ContactDelay,
}

/// Pellet shapes/types used by [`Shot`](Warhead::Shot) [`Warheads`](Warhead)
#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum ShotPellet {
    Spherical,
    Flechette,
}
