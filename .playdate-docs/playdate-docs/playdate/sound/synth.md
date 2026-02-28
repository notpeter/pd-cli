# playdate.sound.synth

## Functions

### playdate.sound.synth.new

```lua
playdate.sound.synth.new(sample: _Sample, sustainStart: integer, sustainEnd: integer): _Synth
playdate.sound.synth.new(waveform: integer): _Synth
```

Returns a new synth object to play a Sample. Sample data must be uncompressed PCM, not ADPCM. An optional sustain region (measured in sample frames) defines a loop to play while the note is active. When the note ends, if an envelope has been set on the synth and the sustain range goes to the end of the sample (i.e. there’s no release section of the sample after the sustain range) then the sustain section continues looping during the envelope release; otherwise it plays through the end of the sample and stops. As a convenience, if `sustainStart` is greater than zero and `sustainEnd` isn’t given, it will be set to the length of the sample.

### playdate.sound.synth:clearEnvelope

```lua
playdate.sound.synth:clearEnvelope(): nil
```

Clears the synth’s envelope settings.

### playdate.sound.synth:copy

```lua
playdate.sound.synth:copy(): _Synth
```

Returns a copy of the given synth.

### playdate.sound.synth:getEnvelope

```lua
playdate.sound.synth:getEnvelope(): _Envelope
```

Returns the synth’s envelope as a playdate.sound.envelope object.

### playdate.sound.synth:getVolume

```lua
playdate.sound.synth:getVolume(): (number, number?)
```

Returns the current volume for the synth, a single value for mono sources or a pair of values (left, right) for stereo sources.
Volume values are between 0.0 and 1.0.

### playdate.sound.synth:isPlaying

```lua
playdate.sound.synth:isPlaying(): boolean
```

Returns true if the synth is still playing, including the release phase of the envelope.

### playdate.sound.synth:noteOff

```lua
playdate.sound.synth:noteOff(): nil
```

Releases the note, if one is playing. The note will continue to be voiced through the release section of the synth’s envelope.

### playdate.sound.synth:playMIDINote

```lua
playdate.sound.synth:playMIDINote(note: (number|string), volume: number, length: number, when: number): boolean
```

Identical to playNote but uses a note name like "C4", or MIDI note number (60=C4, 61=C#4, etc.). In the latter case, fractional values are allowed.

### playdate.sound.synth:playNote

```lua
playdate.sound.synth:playNote(pitch: (number|string), volume: number, length: number, when: number): boolean
```

Plays a note with the current waveform or sample.
* *pitch*: the pitch value is in Hertz. If a sample is playing, pitch=261.63 (C4) plays at normal speed
  * in either function, a string like `Db3` can be used instead of a number
* *volume*: 0 to 1, defaults to 1
* *length*: in seconds. If omitted, note will play until you call noteOff()
* *when*: seconds since the sound engine started (see playdate.sound.getCurrentTime). Defaults to the current time.
The function returns true if the synth was successfully added to the sound channel, otherwise false (i.e., if the channel is full).
If *pitch* is zero, this function calls `noteOff()` instead of potentially adding a non-zero sample, or DC offset, to the output.
Synths currently only have a buffer of one note event. If you call *playNote()* while another note is waiting to play, it will replace that note. To create a sequence of notes to play over a period of time, see playdate.sound.sequence.

### playdate.sound.synth:setADSR

```lua
playdate.sound.synth:setADSR(attack: number, decay: number, sustain: number, release: number, curvature: number): nil
```

Sets the attack time, decay time, sustain level, and release time for the sound envelope, and optionally the curvature.

### playdate.sound.synth:setAmplitudeMod

```lua
playdate.sound.synth:setAmplitudeMod(signal: _Signal): nil
```

Sets the signal to use as the amplitude modulator. Set to *nil* to clear the modulator.

### playdate.sound.synth:setAttack

```lua
playdate.sound.synth:setAttack(time: number): nil
```

Sets the attack time, in seconds.

### playdate.sound.synth:setDecay

```lua
playdate.sound.synth:setDecay(time: number): nil
```

Sets the decay time, in seconds.

### playdate.sound.synth:setEnvelopeCurvature

```lua
playdate.sound.synth:setEnvelopeCurvature(amount: number): nil
```

Smoothly changes the envelope’s shape from linear (amount=0) to exponential (amount=1).

### playdate.sound.synth:setFinishCallback

```lua
playdate.sound.synth:setFinishCallback(_function: function): nil
```

Sets a function to be called when the synth stops playing.

### playdate.sound.synth:setFrequencyMod

```lua
playdate.sound.synth:setFrequencyMod(signal: _Signal): nil
```

Sets the signal to use as the frequency modulator. Set to *nil* to clear the modulator.

### playdate.sound.synth:setLegato

```lua
playdate.sound.synth:setLegato(flag: boolean): nil
```

Sets whether to use legato phrasing for the synth. If the legato flag is set and a new note starts while a previous note is still playing, the synth’s envelope remains in the sustain phase instead of starting a new attack.

### playdate.sound.synth:setParameter

```lua
playdate.sound.synth:setParameter(parameter: integer, value: number): nil
```

Sets the parameter at (1-based) position *num* to the given value. Unless otherwise specified, *value* ranges from 0 to 1.

### playdate.sound.synth:setParameterMod

```lua
playdate.sound.synth:setParameterMod(parameter: integer, signal: _Signal): nil
```

Sets the signal to modulate the parameter. Set to *nil* to clear the modulator.

### playdate.sound.synth:setRelease

```lua
playdate.sound.synth:setRelease(time: number): nil
```

Sets the release time, in seconds.

### playdate.sound.synth:setSustain

```lua
playdate.sound.synth:setSustain(level: number): nil
```

Sets the sustain level, as a proportion of the total level (0.0 to 1.0).

### playdate.sound.synth:setVolume

```lua
playdate.sound.synth:setVolume(left: number, right: number): nil
```

Sets the synth volume. If a single value is passed in, sets both left side and right side volume to the given value. If two values are given, volumes are set separately.
Volume values are between 0.0 and 1.0.

### playdate.sound.synth:setWaveform

```lua
playdate.sound.synth:setWaveform(waveform: integer): nil
```

Sets the waveform or Sample the synth plays. If a sample is given, its data must be uncompressed PCM, not ADPCM. Otherwise *waveform* should be one of the following constants:
* *playdate.sound.kWaveSine*
* *playdate.sound.kWaveSquare*
* *playdate.sound.kWaveSawtooth*
* *playdate.sound.kWaveTriangle*
* *playdate.sound.kWaveNoise*
* *playdate.sound.kWavePOPhase*
* *playdate.sound.kWavePODigital*
* *playdate.sound.kWavePOVosim*

### playdate.sound.synth:setWavetable

```lua
playdate.sound.synth:setWavetable(sample: _Sample, samplesize: integer, xsize: integer, ysize: integer): nil
```

Sets a wavetable for the synth to play. Sample data must be 16-bit mono uncompressed. `samplesize` is the number of samples in each waveform "cell" in the table and must be a power of 2. `xsize` is the number of cells across the wavetable. If the wavetable is two-dimensional, `ysize` gives the number of cells in the y direction.
The synth’s "position" in the wavetable is set manually with setParameter() or automated with setParameterMod(). In some cases it’s easier to use a parameter that matches the waveform position in the table, in others (notably when using envelopes and lfos) it’s more convenient to use a 0-1 scale, so there’s some redundancy here. Parameters are
* 1: x position, values are from 0 to the table width
* 2: x position, values are from 0 to 1, parameter is scaled up to table width
For 2-D tables (`rowwidth` > 0):
* 3: y position, values are from 0 to the table height
* 4: y position, values are from 0 to 1, parameter is scaled up to table height

### playdate.sound.synth:stop

```lua
playdate.sound.synth:stop(): nil
```

Stops the synth immediately, without playing the release part of the envelope.
