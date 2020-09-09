
# Evil Cadastre plots concept rules

The game is divided in a grid of plots (probably 10x10 so coordinates inside a plot are just the last digit)

Players can own a plot when they have the keep inside.

Each player has a certain number of action points (10 + number of plots?).
Total number of action points limited to give negative feedback loop.
Doing something takes an action point.
Possible actions:

- Build a building (multiple turns?)
- Train a unit
- Move a unit within a plot
- Move a unit to an adjacent owned plot
- Use a unit to attack an adjacent hostile plot
- Gather resources
- Move resources to an adjacent owned plot
- fire a cannon (2 turns?)

## Buildings

- Keep (asserts your rule over the plot)
- Production
  - woodcutter
  - quarry
  - iron mine
  - farm
- Stockpile (one for each resource): required to keep any resources. Each stockpile can just hold one resource item. Stockpiles are free to build
- Small Wall: stops invading units in its lane
- Large Wall: stops invading units and can not be broken by normal units
- Guard tower: stops one invading unit in whole plot
- Unit train buildings
  - lair
  - cannon foundry
  - one for each unit?
- Cannon?: destroy buildings; stronger than raider. Possibly a unit instead?
- Trade post: send resources to adjacent friendly plots. One per resource?
- Scout post?: build in adjacent occupied plots. Neccessary to bootstrap the keep and the first storage.

## Units

Units can attack things in adjacent plots like a rook: horizontally or vertically
They continue on the lane (row/column) until they hit a target.
Attacking does not move them: if they survive they'll end up on their old position
Encountering another unit stops them.

- Raider: destroy an enemy building
- Vanguard: distract guards: when a guard would stop another attacking unit it has to fight the attacking vanguard instead
- Warrior: kill units. Also passively kills units that attack it (except other warriors)
- Thief: steal resources from a storage
- Cannon?: destroy an enemy building. Stronger than Raider. Can destroy large walls. Not stopped by guards or warriors. Requires iron for each shot.

## To be decided

Costs for units (how much to build? how much to keep around? how to prevend endless refilling? Does idling cost food? or only attacking?)

Cost for buildings

How common are natural resources (rocks, iron outcrops, forest)?

Should production buildings be on or next to resources?

Production (when there are more plots than actions, does production even matter?)

Taking over plots (how do you build a keep? What happens to units in the plot? Can you take over a plot that has units?)

How strong is a keep?

How do units move within a plot? anywhere? or only horizontally/vertically?
Can multiple units attack on the same lane (probably yes)?

What stops a cannon shot? do units stop it? Do buildings under construction stop it?
Does it destroy things that don't stop it?
