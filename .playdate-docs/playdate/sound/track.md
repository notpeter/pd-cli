# playdate.sound.track

## Functions

### playdate.sound.track.new

```lua
playdate.sound.track.new(): _Track
```

Creates a new `playdate.sound.track` object.

### playdate.sound.track:addControlSignal

```lua
playdate.sound.track:addControlSignal(s: _ControlSignal): nil
```

Adds a playdate.sound.controlsignal object to the track. Note that the signal must be assigned to a modulation input for it to have any audible effect. The input can be anywhere in the sound engine—​it’s not required to belong to the track in any way.

### playdate.sound.track:addNote

```lua
playdate.sound.track:addNote(step: integer, note: (string|integer), length: number, velocity: number): nil
playdate.sound.track:addNote(table: (_SoundTrackNoteIn|_SoundTrackNote)): nil
```

Adds a single note event to the track, letting you specify `step`, `note`, `length`, and `velocity` directly. The second format allows you to pack them into a table, using the format returned by getNotes(). The `note` argument can be a MIDI note number or a note name like "Db3". `length` is the length of the note in steps, not time—​that is, it follows the sequence’s tempo. The default velocity is 1.0.
See setNotes() for the ability to add more than one note at a time.

### playdate.sound.track:clearNotes

```lua
playdate.sound.track:clearNotes(): nil
```

Clears all notes from the track.

### playdate.sound.track:getControlSignals

```lua
playdate.sound.track:getControlSignals(): _ControlSignal[]
```

Returns an array of playdate.sound.controlsignal objects assigned to this track.

### playdate.sound.track:getInstrument

```lua
playdate.sound.track:getInstrument(): _Instrument
```

Gets the playdate.sound.instrument that this track plays.

### playdate.sound.track:getLength

```lua
playdate.sound.track:getLength(): integer
```

Returns the length, in steps, of the track—​that is, the step where the last note in the track ends.

### playdate.sound.track:getNotes

```lua
playdate.sound.track:getNotes(step: integer, endstep: integer): _SoundTrackNote[]
```

Returns an array of tables representing the note events in the track.
The tables contain values for keys `step`, `note`, `length`, and `velocity`. If `step` is given, the function returns only the notes at that step; if both `step` and `endstep` are set, it returns the notes between the two steps (including notes at endstep). n.b. The `note` field in the event tables is always a MIDI note number value, even if the note was added using the string notation.

### playdate.sound.track:getNotesActive

```lua
playdate.sound.track:getNotesActive(): integer
```

Returns the current number of notes active in the track.

### playdate.sound.track:getPolyphony

```lua
playdate.sound.track:getPolyphony(): integer
```

Returns the maximum number of notes simultaneously active in the track. (Known bug: this currently only works for midi files)

### playdate.sound.track:removeNote

```lua
playdate.sound.track:removeNote(step: integer, note: integer): nil
```

Removes the note event at *step* playing *note*.

### playdate.sound.track:setInstrument

```lua
playdate.sound.track:setInstrument(inst: (_Instrument|_Synth)): nil
```

Sets the playdate.sound.instrument that this track plays. If `inst` is a playdate.sound.synth, the function creates an instrument for the synth.

### playdate.sound.track:setMuted

```lua
playdate.sound.track:setMuted(flag: boolean): nil
```

Mutes or unmutes the track.

### playdate.sound.track:setNotes

```lua
playdate.sound.track:setNotes(list: table[]): nil
```

Set multiple notes at once, each array element should be a table containing values for the keys The tables contain values for keys `step`, `note`, `length`, and `velocity`.

## Classes

### _SoundTrackNote

```lua
---@class _SoundTrackNote
---@field step integer
---@field note number
---@field length integer
---@field velocity number
```

### _SoundTrackNoteIn

```lua
---@class _SoundTrackNoteIn
---@field step integer
---@field note (number|string)
---@field length integer
---@field velocity number
```
