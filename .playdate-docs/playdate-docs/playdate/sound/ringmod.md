# playdate.sound.ringmod

## Functions

### playdate.sound.ringmod.new

```lua
playdate.sound.ringmod.new(): _RingMod
```

Creates a new ring modulator filter.

### playdate.sound.ringmod:setFrequency

```lua
playdate.sound.ringmod:setFrequency(f: number): nil
```

Sets the ringmod frequency to *f*.

### playdate.sound.ringmod:setFrequencyMod

```lua
playdate.sound.ringmod:setFrequencyMod(signal: _Signal): nil
```

Sets a signal to modulate the ringmod frequency. Set to *nil* to clear the modulator.

### playdate.sound.ringmod:setMix

```lua
playdate.sound.ringmod:setMix(level: number): nil
```

Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix.

### playdate.sound.ringmod:setMixMod

```lua
playdate.sound.ringmod:setMixMod(signal: _Signal): nil
```

Sets a signal to modulate the mix level. Set to *nil* to clear the modulator.
