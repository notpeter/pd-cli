# playdate.sound.delayline

## Functions

### playdate.sound.delayline.new

```lua
playdate.sound.delayline.new(length: number): _DelayLine
```

Creates a new delay line effect, with the given length (in seconds).

### playdate.sound.delayline:addTap

```lua
playdate.sound.delayline:addTap(delay: number): _DelayLineTap
```

Returns a new playdate.sound.delaylinetap on the delay line, at the given delay (which must be less than or equal to the delay line’s length).

### playdate.sound.delayline:setFeedback

```lua
playdate.sound.delayline:setFeedback(level: number): nil
```

Sets the feedback level of the delay line.

### playdate.sound.delayline:setMix

```lua
playdate.sound.delayline:setMix(level: number): nil
```

Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix, which is useful if you’re using taps for varying delays.

### playdate.sound.delayline:setMixMod

```lua
playdate.sound.delayline:setMixMod(signal: _Signal): nil
```

Sets a signal to modulate the mix level. Set to *nil* to clear the modulator.
