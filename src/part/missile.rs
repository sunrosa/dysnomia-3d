use bevy::prelude::*;

use super::{armor::Armor, engine::Engine, skin::Skin};

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MissileType>()
            .register_type::<Stage>()
            .register_type::<FinalStage>()
            .register_type::<TerminalManeuver>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct MissileType {
    /// The first element represents the "largest" stage, that carries all following stages along with it. The next element represents the missile after first stage separation. The next element represents the missile after second stage separation. Etc.
    stages: Vec<Stage>,
    final_stage: FinalStage,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Stage {
    /// The missile's main engine. Having no engine is optional because of the case of mines (that launch second stages with real engines when detecting a target) and sensor buoys.
    engine: Option<Engine>,

    /// Physical armor to be placed on the surface of the missile
    armor: Option<Armor>,

    /// Coatings such as stealth paint
    skin: Option<Skin>,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct FinalStage {
    /// Payload mass in `kg`.
    payload_mass: f64,

    /// The missile's main engine. Having no engine is optional because of the case of mines (that launch second stages with real engines when detecting a target) and sensor buoys.
    main_engine: Option<Engine>,

    /// Physical armor to be placed on the surface of the missile
    armor: Option<Armor>,

    /// Coatings such as stealth paint
    skin: Option<Skin>,

    /// The missile's engine to be used for course corrections at final approach to increase the chance of success, and for [`terminal avoidance maneuvers`](TerminalManeuver) to increase point-defense avoidance chances.
    terminal_reaction_engine: Option<Engine>,
    /// Maneuver to initiate at [`terminal_maneuver_range`](Self::terminal_maneuver_range) away from the target. This will expend more fuel and slow the approach, however potentially reduce the chance of point-defense interception.
    terminal_maneuver: TerminalManeuver,
    /// Distance in `m` away from the target where [`terminal maneuvers`](Self::terminal_maneuver) shall be initiated, and shall continue indefinitely.
    terminal_maneuver_range: f64,
    /// Velocity in `m/s` at which to intercept target. Higher speeds reduce maneuver time, reduce chance of successful retargeting and/or u-turn, but also reduce the chance of interception.
    terminal_velocity: f64,
    /// Whether the missile should decelerate to reach desired [`terminal_velocity`](Self::terminal_velocity) or not. Deceleration becomes optionally necessary if the launching ship is traveling at a higher speed relative to the target than the desired [`terminal_velocity`](Self::terminal_velocity).
    decelerate_to_terminal: bool,
}

/// At terminal approach, these maneuvers are used to increase point-defense avoidance chances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum TerminalManeuver {
    None,
    Weave,
    Corkscrew,
    Chaos,
}

impl Default for TerminalManeuver {
    fn default() -> Self {
        TerminalManeuver::None
    }
}
