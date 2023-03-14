# Dungeon Crawler Design Notes

## Project Name (Working):

- Dungeon Rougelike

## Description

A dungeon crawler rougelike with procedurally generated levels and monsters, increasing levels of difficulty, and turn based movement.

## Story

The hero’s hometown is suffering from a plague of monsters. Welling up from the deep,
they seem unstoppable. Legend tells of the Amulet of Yala - Yet Another Lost Amulet -
that can be used to stem the tide. After a long night at the tavern, the hero promises to
save the day - and sets forth into the dungeon.

## Game Loop

- Enter dungeon level
- Explore, revealing the map
- Encounter enemies, fight or flee from them
- Find power-ups, use them to strengthen the player
- Locate the level exit
- Repeat

## MVP

- Create a basic dungeon map
- Place the player and let them walk around
- Spawn Monsters, draw them, and let the player kill them by walking into them.
- Add health and a combat system
- Add healing potions
- Display a game over screen when the player dies
- Add the Amulet of Yala to the level and let the player win by reaching it

## Stretch Goals

- Add Fields of View
- Add more interesting Dungeon Designs
- Add some dungeon themes
- Add multiple layers to the dungeon, with the Amulet on the last one
- Add varied weapons to the game
- Move to a data driven design for spawning enemies
- Consider some visual effects to make combat more impactful
- Consider keeping score
