# playdate.sound.bitcrusher

## Functions

### playdate.sound.bitcrusher.new

```lua
playdate.sound.bitcrusher.new(): _BitCrusher
```

Creates a new bitcrusher filter.

### playdate.sound.bitcrusher:setAmount

```lua
playdate.sound.bitcrusher:setAmount(amt: number): nil
```

Sets the amount of crushing to *amt*. Valid values are 0 (no effect) to 1 (quantizing output to 1-bit).

### playdate.sound.bitcrusher:setAmountMod

```lua
playdate.sound.bitcrusher:setAmountMod(signal: _Signal): nil
```

Sets a signal to modulate the filter level. Set to *nil* to clear the modulator.

### playdate.sound.bitcrusher:setMix

```lua
playdate.sound.bitcrusher:setMix(level: number): nil
```

Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix.

### playdate.sound.bitcrusher:setMixMod

```lua
playdate.sound.bitcrusher:setMixMod(signal: _Signal): nil
```

Sets a signal to modulate the mix level. Set to *nil* to clear the modulator.

### playdate.sound.bitcrusher:setUndersampling

```lua
playdate.sound.bitcrusher:setUndersampling(amt: number): nil
```

Sets the number of samples to repeat; 0 is no undersampling, 1 effectively halves the sample rate.

### playdate.sound.bitcrusher:setUndersamplingMod

```lua
playdate.sound.bitcrusher:setUndersamplingMod(signal: _Signal): nil
```

Sets a signal to modulate the filter level. Set to *nil* to clear the modulator.
