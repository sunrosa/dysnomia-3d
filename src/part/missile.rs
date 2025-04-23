use bevy::prelude::*;

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MissileType>()
            .register_type::<MissileStage>()
            .register_type::<TerminalManeuver>();
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct MissileType {
    /// The first element represents the "largest" stage, that carries all following stages along with it. The next element represents the missile after first stage separation. The next element represents the missile after second stage separation. Etc.
    stages: Vec<MissileStage>,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct MissileStage {
    /// Payload mass in `kg`.
    payload_mass: f64,

    /// Maneuver to initiate at [`terminal_maneuver_range`](Self::terminal_maneuver_range) away from the target. This will expend more fuel and slow the approach, however potentially reduce the chance of point-defense interception.
    terminal_maneuver: TerminalManeuver,
    /// Distance in `m` away from the target where [`terminal maneuvers`](Self::terminal_maneuver) shall be initiated, and shall continue indefinitely.
    terminal_maneuver_range: f64,
}

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
