# playdate.sound.sequence

## Functions

### playdate.sound.sequence.new

```lua
playdate.sound.sequence.new(midi_path: string): _Sequence
```

Creates a new sound sequence. If `midi_path` is given, it attempts to load data from the midi file into the sequence.

### playdate.sound.sequence:addTrack

```lua
playdate.sound.sequence:addTrack(track: _Track): nil
```

Adds the given playdate.sound.track to the sequence. If `track` omitted, the function creates and returns a new track.

### playdate.sound.sequence:allNotesOff

```lua
playdate.sound.sequence:allNotesOff(): nil
```

Sends an allNotesOff() message to each track’s instrument.

### playdate.sound.sequence:getCurrentStep

```lua
playdate.sound.sequence:getCurrentStep(): number
```

Returns the step number the sequence is currently at.

### playdate.sound.sequence:getLength

```lua
playdate.sound.sequence:getLength(): number
```

Returns the length of the longest track in the sequence, in steps. See also playdate.sound.track.getLength().

### playdate.sound.sequence:getTempo

```lua
playdate.sound.sequence:getTempo(): number
```

Returns the tempo of the sequence, in steps per second.

### playdate.sound.sequence:getTrackAtIndex

```lua
playdate.sound.sequence:getTrackAtIndex(n: integer): _Track
```

Returns the playdate.sound.track object at position `n` in the sequence.

### playdate.sound.sequence:getTrackCount

```lua
playdate.sound.sequence:getTrackCount(): integer
```

Returns the number of tracks in the sequence.

### playdate.sound.sequence:goToStep

```lua
playdate.sound.sequence:goToStep(step: integer, play: boolean): nil
```

Moves the play position for the sequence to step number `step`. If `play` is set, triggers the notes at that step.

### playdate.sound.sequence:isPlaying

```lua
playdate.sound.sequence:isPlaying(): boolean
```

Returns true if the sequence is currently playing.

### playdate.sound.sequence:play

```lua
playdate.sound.sequence:play(finishCallback: fun(self: _Sequence): nil): nil
```

Starts playing the sequence. `finishCallback` is an optional function to be called when the sequence finishes playing or is stopped. The sequence is passed to the callback as its single argument.

### playdate.sound.sequence:setLoops

```lua
playdate.sound.sequence:setLoops(loopCount: integer): nil
playdate.sound.sequence:setLoops(startStep: integer, endStep: integer, loopCount: integer): nil
```

Same as above, with startStep set to 0 and endStep set to `sequence:getLength()`.

### playdate.sound.sequence:setTempo

```lua
playdate.sound.sequence:setTempo(stepsPerSecond: number): nil
```

Sets the tempo of the sequence, in steps per second.

### playdate.sound.sequence:setTrackAtIndex

```lua
playdate.sound.sequence:setTrackAtIndex(n: integer, track: _Track): nil
```

Sets the given playdate.sound.track object at position `n` in the sequence.

### playdate.sound.sequence:stop

```lua
playdate.sound.sequence:stop(): nil
```

Stops playing the sequence.
