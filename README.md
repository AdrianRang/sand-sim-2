# Sand Simulator
Please ignore the 2

This 'simulates' sand water and stone although it is not meant in any way to follow the laws of physics

![sim gif](./sim.gif)

## How it works
Everything is a particle of any of the 3 types available: 
- Stone, does nothing
- Sand, If nothing is down goes down, else if nothing is right-down goes right-down else does the same but for down-left
- Water, It follows the same rules as sand but if it can't do nothing and theres a free space either left or right it goes there.

These really simple rules give us the expected behaviour.

## How to use
Left-click - sand at the mouse
Right-click - destroy at the mouse
Shift+Left-Click - Wall at the mouse
W - Water at the mouse
R - Rain
S - Sand as Rain

### Known issues
- Water can rise through stone
- Water can rise to out of bounds where the program crashes
- It gets really slow when a lot of particles are present
- No frame timing
  
### Planned features
- Toggleable solid blocks
- Sizing Menu