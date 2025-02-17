#set par(justify: true)
#set text(size: 13pt)
#set page(paper: "a4", margin: 6.5%)

#columns(2, gutter: 3.5%)[
= Design fixtures
- n-body astrodynamics (calculated with DOPRI) shall influence all aspects of the game. Tactics and strategies alike shall center themselves upon ship and projectile trajectories.
  - For example, the force (and damaging capability) behind a projectile is mostly influenced by the velocity of the attacking ship relative to the defending ship.
- Fuel shall add a much-needed restriction on the player's astrodynamic maneuvering choices. Every maneuver shall be costly, and there shall be a tightly limited amount that can be performed before refueling.
- Ships shall be 3-dimensional, like the space they inhabit. Projectile impacts shall take a ship's hull into consideration, as well as its systems and subsystems, which will be located in specific regions of a ship. The player shall be the one to design their ships, inside a range of pre-made hulls, by choosing modular systems and subsystems, and then tuning those modules. The player shall be able to design those modules as well, which will have the same design complexity of ships themselves.
- Weapons, munitions, and missiles shall also be available for intricate player design.

= Design considerations
- The game should be turn-based to allow for deep calculations to take place without hurting user experience. An example of a live spaceship skirmishing game is Nebulous: Fleet Command. It suffers large performance hits even in relatively small games. And the simulation depth of Dysnomia 3D shall be far greater than Nebulous's.
]
