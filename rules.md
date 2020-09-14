
# Rules



# Reference

## Entities

	pub enum Entity {
		
		Keep(UserId),
		
		Construction(BuildingType),
		
		// Units
		Raider,
		Warrior,
		
		// Production buildings
		Farm,
		Woodcutter,
		Quarry,
		// Unit training buildings
		Lair,
		Barracks,
		// Special buildings
		Stockpile(Option<Resource>),
		Road,
		Tradepost,
		Scoutpost,
		
		// Ambient
		Forest,
		Swamp,
		Rock,
	}

The Construction entity is currently unused.

### Keep

The keep is the most important building of a plot.
The keep decides who owns the plot.
For each plot there is always one location in the center where the keep is located. This also means that there can not be 2 keeps in one plot.
All other commands determine who owns the plot by looking at the keep.

A player that does not have any keeps can create the first one with a Claim action in a plot.
Players can conquer/construct a keep in an adjacent plot with a scout post.
This is only possible when there are no units in the plot that is to be taken over.

### Stockpile

Can hold a resource.
Empty stockpiles can be built for free with the Build command.
Obtaining a resource in the plot can turn an empty stockpile into a stockpile containing a resource.
Paying for something can turn a resource containing stockpile into an empty stockpile
Resources can only be used within the same plot.
Resources can be moved to another empty stockpile in the plot, or to another plot using a tradepost.

### Units

Units can attack adjacent plots.
When the attack action is given with a direction they will attack the lane in the adjacent plot.

Units can also move to any empty tile within the same plot.
Units can move to adjacent plots owned by the same player using roads.

Units prevent hostile scoutposts from taking over the plot.


#### Raider

Raiders can destroy buildings.
They will move in the lane they attack until they encounter a building or another unit.

Raiders can destroy Woodcutter, Quarry, Farm, Lair, Barracks and Scoutpost.
Raiders will also destoy any Road or Tradepost they encounter on their way. This will not stop them.
Raiders can move through Forest, Swamp and Rock.
Raiders do not interact with Stockpiles.

Raiders are trained at a Lair.

#### Warrior

Warriors can kill other units
They are not stopped by buildings

Warriors are trained at a barracks

### Production buildings

Production buildings will give you a resource when you use them.
The resource will be placed in the nearest free stockpile in the plot.
If there is no free stockpile then nothing happens.

#### Farm

Produces food

#### Woodcutter

Produces wood.
Can only be built next to Forest.
Free to build

#### Quarry

Produces stone
Can only be built next to Rock

### Training buildings

Produces a unit when you use them.
This does cost some resources.
The unit will appear on the nearest free tile in the plot.

#### Lair

Produces Raiders

#### Barracks

Produces Warriors

### Scoutpost

Can only be placed on the edge of a plot.
When used it tries to take over the plot it borders.
The player has to pay resources for this.
Taking over a plot is only possible if there are no units in that plot.

### Road

Can only be placed on the edge of a plot.
When a unit moves onto the road and the bordering plot belongs to the same player then the unit goes to that plot
The unit will be placed on the nearest free place to the road.


### Tradepost

Can only be placed on the edge of a plot.
When a resource moves onto the tradepost and the bordering plot belongs to the same player then the resource goes to that plot
The resource will be placed on the nearest free stockpile to the tradepost.

### Ambient

These tiles can only be created at world generation. They can not be destroyed.
It is not possible to build on these tiles
Units can pass through these tiles without problem

#### Swamp

#### Forest

Woodcutters can only be built next to a forest

#### Rock

Quarries can only be built next to a rock

## Actions


	pub enum Action{
		Claim,
		Build(BuildingType),
		Move(Pos),
		Attack(Direction),
		Use,
		Remove,
	}

Each action has an associated user and position.
Except for the Claim action, all actions must be done on plots that are already controlled by the player.

### Claim

Claim a keep that is unclaimed and does not border any claimed keep.
Only possible when you have no keeps.

### Build

Create a new building on an empty tile, and pay the cost for that building.
Additional restriction may apply, such as being on the plot border, or bordering a forest/rock.

### Move

Move a unit or resource.
Units can move to empty tiles, resources to empty stockpiles.
Units can also move to roads, and resources to tradeposts to move to an adjacent owned plot.

### Attack

Attack an adjacent hostile plot.
All attacks are evaluated after checking the actions, so if the attacked entity had an action at the same time as the attack then that action would still be completed.

### Use

Use can mean different things depending on the building

#### Produce a resource

For production buildings using them will produce their associated resource

#### Produce a unit

For training buildings using them will train a unit (after paying the cost)

#### Take over an adjacent plot

For scoutposts using them will take over the plot they are bordering (after paying the cost). Only possible when there are no units in that plot.

### Remove

Remove a building to free the tile.
