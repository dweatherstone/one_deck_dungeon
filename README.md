# One Deck Dungeon

## Current Bugs

1. In requirements and consequences for skills, need to differentiate between quantity of dice, and value of dice.
2. Add default for the creation of Attributes.
3. Clean up the errors caused by numbers 1 and 2 above.
4. Need to look at the Effect types. They don't seem to be fit for purpose at the moment, and can only be explained by reading the text. It may be that they each skill needs its own function, and then the hero (or the game) struct contains a Vec of Skills that the current player has.
