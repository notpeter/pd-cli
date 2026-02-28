# playdate.sound.overdrive

## Functions

### playdate.sound.overdrive.new

```lua
playdate.sound.overdrive.new(): _OverDrive
```

Creates a new overdrive effect.

### playdate.sound.overdrive:setGain

```lua
playdate.sound.overdrive:setGain(level: number): nil
```

Sets the gain of the filter.

### playdate.sound.overdrive:setLimit

```lua
playdate.sound.overdrive:setLimit(level: number): nil
```

Sets the level where the amplified input clips.

### playdate.sound.overdrive:setLimitMod

```lua
playdate.sound.overdrive:setLimitMod(signal: _Signal): nil
```

Sets a signal to modulate the limit level. Set to *nil* to clear the modulator.

### playdate.sound.overdrive:setMix

```lua
playdate.sound.overdrive:setMix(level: number): nil
```

Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix.

### playdate.sound.overdrive:setMixMod

```lua
playdate.sound.overdrive:setMixMod(signal: _Signal): nil
```

Sets a signal to modulate the mix level. Set to *nil* to clear the modulator.

### playdate.sound.overdrive:setOffset

```lua
playdate.sound.overdrive:setOffset(level: number): nil
```

Adds an offset to the upper and lower limits to create an asymmetric clipping.

### playdate.sound.overdrive:setOffsetMod

```lua
playdate.sound.overdrive:setOffsetMod(signal: _Signal): nil
```

Sets a signal to modulate the offset value. Set to *nil* to clear the modulator.
