# playdate.sound.delaylinetap

## Functions

### playdate.sound.delaylinetap:getVolume

```lua
playdate.sound.delaylinetap:getVolume(): number
```

Returns the tap’s volume.

### playdate.sound.delaylinetap:setDelay

```lua
playdate.sound.delaylinetap:setDelay(time: number): nil
```

Sets the position of the tap on the delay line, up to the delay line’s length.

### playdate.sound.delaylinetap:setDelayMod

```lua
playdate.sound.delaylinetap:setDelayMod(signal: _Signal): nil
```

Sets a signal to modulate the tap delay. If the signal is continuous (e.g. an envelope or a triangle LFO, but not a square LFO) playback is sped up or slowed down to compress or expand time. Set to *nil* to clear the modulator.

### playdate.sound.delaylinetap:setFlipChannels

```lua
playdate.sound.delaylinetap:setFlipChannels(flag: boolean): nil
```

If set and the delay line is stereo, the tap outputs the delay line’s left channel to its right output and vice versa.

### playdate.sound.delaylinetap:setVolume

```lua
playdate.sound.delaylinetap:setVolume(level: number): nil
```

Sets the tap’s volume.
