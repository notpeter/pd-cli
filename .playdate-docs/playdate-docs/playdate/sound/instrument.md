# playdate.sound.instrument

## Functions

### playdate.sound.instrument.new

```lua
playdate.sound.instrument.new(synth: _Synth): _Instrument
```

Creates a new `playdate.sound.instrument` object. If `synth` is given, adds it as a voice for the instrument.

### playdate.sound.instrument:addVoice

```lua
playdate.sound.instrument:addVoice(v: _Synth, note: integer, rangeend: integer, transpose: integer): nil
```

Adds the given playdate.sound.synth to the instrument. If only the *note* argument is given, the voice is only used for that note, and is transposed to play at normal speed (i.e. rate=1.0 for samples, or C4 for synths). If *rangeend* is given, the voice is assigned to the range *note* to *rangeend*, inclusive, with the first note in the range transposed to rate=1.0/C4. The `note` and `rangeend` arguments can be MIDI note numbers or note names like "Db3". The final transpose argument transposes the note played, in half-tone units.

### playdate.sound.instrument:allNotesOff

```lua
playdate.sound.instrument:allNotesOff(): nil
```

Sends a stop signal to all playing notes.

### playdate.sound.instrument:getVolume

```lua
playdate.sound.instrument:getVolume(): (number, number?)
```

Returns the current volume for the synth, a single value for mono sources or a pair of values (left, right) for stereo sources.
Volume values are between 0.0 and 1.0.

### playdate.sound.instrument:noteOff

```lua
playdate.sound.instrument:noteOff(note: integer, when: number): nil
```

Stops the instrument voice playing note *note*. If *when* is given, the note is stopped *when* seconds in the future, otherwise it’s stopped immediately.

### playdate.sound.instrument:playMIDINote

```lua
playdate.sound.instrument:playMIDINote(note: number, vel: number, length: number, when: number): nil
```

Identical to `instrument:playNote()` but *note* is a MIDI note number: 60=C4, 61=C#4, etc. Fractional values are allowed.

### playdate.sound.instrument:playNote

```lua
playdate.sound.instrument:playNote(frequency: string, vel: number, length: number, when: number): nil
```

Plays the given note on the instrument. A string like `Db3` can be used instead of a pitch/note number. Fractional values are allowed. *vel* defaults to 1.0, fully on. If *length* isn’t specified, the note stays on until *instrument.noteOff(note)* is called. *when* is the number of seconds in the future to start playing the note, default is immediately.

### playdate.sound.instrument:setPitchBend

```lua
playdate.sound.instrument:setPitchBend(amount: number): nil
```

Sets the pitch bend to be applied to the voices in the instrument, as a fraction of the full range.

### playdate.sound.instrument:setPitchBendRange

```lua
playdate.sound.instrument:setPitchBendRange(halfsteps: number): nil
```

Sets the pitch bend range for the voices in the instrument. The default range is 12, for a full octave.

### playdate.sound.instrument:setTranspose

```lua
playdate.sound.instrument:setTranspose(halfsteps: number): nil
```

Transposes all voices in the instrument. *halfsteps* can be a fractional value.

### playdate.sound.instrument:setVolume

```lua
playdate.sound.instrument:setVolume(left: number, right: number): nil
```

Sets the instrument volume. If a single value is passed in, sets both left side and right side volume to the given value. If two values are given, volumes are set separately.
Volume values are between 0.0 and 1.0.
