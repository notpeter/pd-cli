# playdate.sound.lfo

## Functions

### playdate.sound.lfo.new

```lua
playdate.sound.lfo.new(type: integer): _LFO
```

Returns a new LFO object, which can be used to modulate sounds. See playdate.sound.lfo:setType() for LFO types.

### playdate.sound.lfo:getValue

```lua
playdate.sound.lfo:getValue(): number
```

Returns the current signal value of the LFO.

### playdate.sound.lfo:setArpeggio

```lua
playdate.sound.lfo:setArpeggio(note1: number, ...: number): nil
```

Sets the LFO type to arpeggio, where the given values are in half-steps from the center note. For example, the sequence (0, 4, 7, 12) plays the notes of a major chord.

### playdate.sound.lfo:setCenter

```lua
playdate.sound.lfo:setCenter(center: number): nil
```

Sets the center value of the LFO.

### playdate.sound.lfo:setDelay

```lua
playdate.sound.lfo:setDelay(holdoff: number, ramp: number): nil
```

Sets an initial holdoff time for the LFO where the LFO remains at its center value, and a ramp time where the value increases linearly to its maximum depth. Values are in seconds.

### playdate.sound.lfo:setDepth

```lua
playdate.sound.lfo:setDepth(depth: number): nil
```

Sets the depth of the LFO’s modulation.

### playdate.sound.lfo:setGlobal

```lua
playdate.sound.lfo:setGlobal(flag: boolean): nil
```

If an LFO is marked global, it is continuously updated whether or not it’s attached to any source.

### playdate.sound.lfo:setPhase

```lua
playdate.sound.lfo:setPhase(phase: number): nil
```

Sets the current phase of the LFO, from 0 to 1.

### playdate.sound.lfo:setRate

```lua
playdate.sound.lfo:setRate(rate: number): nil
```

Sets the rate of the LFO, in cycles per second.

### playdate.sound.lfo:setRetrigger

```lua
playdate.sound.lfo:setRetrigger(flag: boolean): nil
```

If retrigger is on, the LFO’s phase is reset to its initial phase (default 0) when a synth using the LFO starts playing a note.

### playdate.sound.lfo:setStartPhase

```lua
playdate.sound.lfo:setStartPhase(phase: number): nil
```

Sets the initial phase of the LFO, from 0 to 1.

### playdate.sound.lfo:setType

```lua
playdate.sound.lfo:setType(type: integer): nil
```

Sets the waveform of the LFO. Valid values are
* *playdate.sound.kLFOSquare*
* *playdate.sound.kLFOSawtoothUp*
* *playdate.sound.kLFOSawtoothDown*
* *playdate.sound.kLFOTriangle*
* *playdate.sound.kLFOSine*
* *playdate.sound.kLFOSampleAndHold*

### playdate.sound.lfo:setOffset

```lua
playdate.sound.lfo:setOffset(offset: number): nil
```

### playdate.sound.lfo:setScale

```lua
playdate.sound.lfo:setScale(scale: number): nil
```
