# playdate.sound.onepolefilter

## Functions

### playdate.sound.onepolefilter.new

```lua
playdate.sound.onepolefilter.new(): _OnePoleFilter
```

Returns a new one pole filter.

### playdate.sound.onepolefilter:setMix

```lua
playdate.sound.onepolefilter:setMix(level: number): nil
```

Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix.

### playdate.sound.onepolefilter:setMixMod

```lua
playdate.sound.onepolefilter:setMixMod(signal: _Signal): nil
```

Sets a signal to modulate the mix level. Set to *nil* to clear the modulator.

### playdate.sound.onepolefilter:setParameter

```lua
playdate.sound.onepolefilter:setParameter(p: number): nil
```

Sets the filter’s single parameter (cutoff frequency) to *p*.

### playdate.sound.onepolefilter:setParameterMod

```lua
playdate.sound.onepolefilter:setParameterMod(m: _Signal): nil
```

Sets a modulator for the filter’s parameter. Set to *nil* to clear the modulator.
