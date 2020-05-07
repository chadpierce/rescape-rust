# rescape

- rescape is a roguelike written in Rust
- rescape is a Nethack-like game without all of the spoilers and secrets (no kitchen sink)
- As of this writing Rescape is far from finished and is a very basic proof of concept
- I am not a developer and probably do not write good and efficient code

## Gameplay

The name rescape came from the idea that you would escape from a dungeon rather than begin on the ground floor as an ambitious hero seeking glory and treasure. The name is short for “rogue escape”.

Your character starts at the bottom of a dungeon where you were imprisoned by the Wizard of Yendor (a roguelike trope). The goal is to escape the dungeon, and then if you so choose, climb the Wizard’s tower to defeat it and take the Amulet of Yendor (another trope) as your reward. 

Other basics:
- The hero has no race or sex
- There are 3 classes (rogue, fighter, wizard) with unique abilities and traits, along with standard attributes like strength, dexterity, and intelligence
- There are 2 religions the hero can follow with unique benefits
- There are weapons, armor, magical items, and spells
- There are monsters that will try to kill you
- There are branches, including the optional Wizard tower endgame
- The hero does not “level up” and there is no XP
- Certain items can be blessed by your god to permanently raise stats upon their use - for example: a blessed potion of healing raises your maximum hit points instead healing your current health level

## Notes

- rescape is the first Rust code I’ve written - and the largest coding project I’ve ever worked on.
- Because I am not a developer the code is probably bad
- No libraries are being used for game logic, only for keyboard input and output to the screen
- rescape will currently only compile on a *nix - this is not likely change
- The goal is not to finish the game, but to learn a modern language and have fun
- The first few parts of this tutorial were loosely followed to get started: https://tomassedovic.github.io/roguelike-tutorial
- Development began in early 2020, but after COVID-19 working on it has become a nice escape while sheltering in place - no pun intended
