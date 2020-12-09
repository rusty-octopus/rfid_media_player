# Media Player

## Design / Todo

* Simple interface: `play`
* Handles not playing track twice
* Interrupts track when new track is required
* Knows when a track is playing (in order to play a track twice when it already finished)
* Uses medialib trait internally (in order to change to some different lib)
* NewType for Track (from String etc.)