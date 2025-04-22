#set par(justify: true)
#set text(size: 14pt)
#set page(paper: "a4", margin: 5%)

#columns(2, gutter: 3.5%)[
= Design fixtures
- n-body astrodynamics (calculated with Dormand-Prince ODE Method if the processing power can be spared, or otherwise Barnes-Hut (Costzones?) Method and/or Fast Multipole Method) shall influence all aspects of the game. Tactics and strategies alike shall center themselves upon ship and projectile trajectories.
  - Planetary masses shall each influence each of each other simultaneously. Ships shall not influence planets (be massless). Asteroids and comets may or may not influence planets, depending on processing power available.
  - The force (and damaging capability) behind a projectile is mostly influenced by the velocity of the attacking ship relative to the defending ship. Gravity assists shall be performable on projectiles, just as they are performable for ships.
- Fuel shall add a much-needed restriction on the player's astrodynamic maneuvering choices. Every maneuver shall be costly. There shall be a tightly limited delta-V that can be exerted before refueling.
// - Ships shall be 3-dimensional, like the space they inhabit. Projectile impacts shall take a ship's hull into consideration, as well as its systems and subsystems, which will be located in specific regions of a ship. The player shall be the one to design their ships, inside a range of pre-made hulls, by choosing modular systems and subsystems, and then tuning those modules. The player shall be able to design those modules as well, which will have the same design complexity of ships themselves.
- The components inside of ships shall not be subject to a simple damage-allocation-chart (DAC). Players shall instead be able to place components in fortified positions of a ship (such as the rear, if the ship is designed for bow-tanking), and the orientation of ships shall be subject to player control mid-battle, to make tactics like bow-tanking possible and flexible.
- Weapons, munitions, and missiles shall be available for intricate player design.

= Design considerations
- The game should be turn-based to allow for deep calculations to take place without hurting user experience. An example of a live spaceship skirmishing game is Nebulous: Fleet Command. It suffers large performance hits even in relatively small games. And the simulation depth of Dysnomia 3D shall be far greater than Nebulous's.

= Inspirations
Dysnomia 3D is inspired by Cogmind's turn-based simulations as well as its modular part builds, combined with the nature of Aurora 4x, with combat similar to Nebulous: Fleet Command, with overarching lore from Dysnomia#footnote[Dysnomia is my original work.], presented to the player in bits and pieces, as Cogmind does.

= Game modes
For Dysnomia's lore to be properly presented, singleplayer and multiplayer co-op shall be the available campaign modes. An intent is to also have a campaign mode with no built-in story, akin to 4x games, which would allow for multiplayer PVP and potentially singleplayer against AI. A focus shall be placed on adding a "skirmish" mode, where players create a battle and then fight it immediately (against AI or players).

Multiplayer PVP is a high priority.
]
