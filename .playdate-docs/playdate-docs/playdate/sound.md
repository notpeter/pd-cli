# playdate.sound

## Functions

### playdate.sound.addEffect

```lua
playdate.sound.addEffect(effect: _SoundEffect): nil
```

Adds the given playdate.sound.effect to the default sound channel.

### playdate.sound.getHeadphoneState

```lua
playdate.sound.getHeadphoneState(changeCallback: fun(headphones: boolean, mic:boolean): nil): (boolean, boolean)
```

Returns a pair of booleans (headphone, mic) indicating whether headphones are plugged in, and if so whether they have a microphone attached. If *changeCallback* is a function, it will be called every time the headphone state changes, until it is cleared by calling `playdate.sound.getHeadphoneState(nil)`. If a change callback is set, the audio does **not** automatically switch from speaker to headphones when headphones are plugged in (and vice versa), so the callback should use `playdate.sound.setOutputsActive()` to change the output if needed. The callback is passed two booleans, matching the return values from `getHeadphoneState()`: the first `true` if headphones are connect, and the second `true` if the headphones have a microphone.
Equivalent to `playdate->sound->getHeadphoneState()` in the C API.

### playdate.sound.getSampleRate

```lua
playdate.sound.getSampleRate(): integer
```

Returns the sample rate of the audio system (44100). The sample rate is determined by the hardware, and is not currently mutable.

### playdate.sound.playingSources

```lua
playdate.sound.playingSources(): _SoundSource[]
```

Returns a list of all sources currently playing.

### playdate.sound.removeEffect

```lua
playdate.sound.removeEffect(effect: _SoundEffect): nil
```

Removes the given effect from the default sound channel.

### playdate.sound.setOutputsActive

```lua
playdate.sound.setOutputsActive(headphones: boolean, speaker: boolean): nil
```

Forces sound to be played on the headphones or on the speaker, regardless of whether headphones are plugged in or not. (With the caveat that it is not actually possible to play on the headphones if they’re not plugged in.) This function has no effect in the Simulator.
Equivalent to `playdate->sound->setOutputsActive()` in the C API.

## Classes

### playdate.sound

```lua
---@class playdate.sound
---@field kFilterLowPass integer 0
---@field kFilterHighPass integer 1
---@field kFilterBandPass integer 2
---@field kFilterNotch integer 3
---@field kFilterPEQ integer 4
---@field kFilterLowShelf integer 5
---@field kFilterHighShelf integer 6
---@field kFormat8bitMono integer 0
---@field kFormat8bitStereo integer 1
---@field kFormat16bitMono integer 2
---@field kFormat16bitStereo integer 3
---@field kLFOSquare integer 0
---@field kLFOTriangle integer 1
---@field kLFOSine integer 2
---@field kLFOSampleAndHold integer 3
---@field kLFOSawtoothUp integer 4
---@field kLFOSawtoothDown integer 5
---@field kWaveSquare integer 0
---@field kWaveTriangle integer 1
---@field kWaveSine integer 2
---@field kWaveNoise integer 3
---@field kWaveSawtooth integer 4
---@field kWavePOPhase integer 5
---@field kWavePODigital integer 6
---@field kWavePOVosim integer 7
```

## See Also:

- [bitcrusher](sound/bitcrusher.md)
- [channel](sound/channel.md)
- [controlsignal](sound/controlsignal.md)
- [delayline](sound/delayline.md)
- [delaylinetap](sound/delaylinetap.md)
- [envelope](sound/envelope.md)
- [fileplayer](sound/fileplayer.md)
- [instrument](sound/instrument.md)
- [lfo](sound/lfo.md)
- [micinput](sound/micinput.md)
- [onepolefilter](sound/onepolefilter.md)
- [overdrive](sound/overdrive.md)
- [ringmod](sound/ringmod.md)
- [sample](sound/sample.md)
- [sampleplayer](sound/sampleplayer.md)
- [sequence](sound/sequence.md)
- [signal](sound/signal.md)
- [signalvalue](sound/signalvalue.md)
- [synth](sound/synth.md)
- [track](sound/track.md)
- [twopolefilter](sound/twopolefilter.md)
