# playdate.sound.twopolefilter

## Functions

### playdate.sound.twopolefilter.new

```lua
playdate.sound.twopolefilter.new(type: (integer|string)): _TwoPoleFilter
```

Creates a new two pole IIR filter of the given *type*:
* *playdate.sound.kFilterLowPass* (or the string "lowpass" or "lopass")
* *playdate.sound.kFilterHighPass* (or "highpass" or "hipass")
* *playdate.sound.kFilterBandPass* (or "bandpass")
* *playdate.sound.kFilterNotch* (or "notch")
* *playdate.sound.kFilterPEQ* (or "peq")
* *playdate.sound.kFilterLowShelf* (or "lowshelf" or "loshelf")
* *playdate.sound.kFilterHighShelf* (or "highshelf" or "hishelf")

### playdate.sound.twopolefilter:setFrequency

```lua
playdate.sound.twopolefilter:setFrequency(f: number): nil
```

Sets the center frequency (in Hz) of the filter to *f*.

### playdate.sound.twopolefilter:setFrequencyMod

```lua
playdate.sound.twopolefilter:setFrequencyMod(signal: _Signal): nil
```

Sets a signal to modulate the filter frequency. Set to *nil* to clear the modulator.

### playdate.sound.twopolefilter:setGain

```lua
playdate.sound.twopolefilter:setGain(g: number): nil
```

Sets the gain of the filter to *g*. Gain is only used in PEQ and shelf type filters.

### playdate.sound.twopolefilter:setMix

```lua
playdate.sound.twopolefilter:setMix(level: number): nil
```

Sets the wet/dry mix for the effect. A level of 1 (full wet) replaces the input with the effect output; 0 leaves the effect out of the mix.

### playdate.sound.twopolefilter:setMixMod

```lua
playdate.sound.twopolefilter:setMixMod(signal: _Signal): nil
```

Sets a signal to modulate the mix level. Set to *nil* to clear the modulator.

### playdate.sound.twopolefilter:setResonance

```lua
playdate.sound.twopolefilter:setResonance(r: number): nil
```

Sets the resonance of the filter to *r*. Valid values are in the range 0-1. This parameter has no effect on shelf type filters.

### playdate.sound.twopolefilter:setResonanceMod

```lua
playdate.sound.twopolefilter:setResonanceMod(signal: _Signal): nil
```

Sets a signal to modulate the filter resonance. Set to *nil* to clear the modulator.

### playdate.sound.twopolefilter:setType

```lua
playdate.sound.twopolefilter:setType(type: (integer|string)): nil
```

Sets the type of the filter to *type*.
