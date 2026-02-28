# playdate.sound.sampleplayer

## Functions

### playdate.sound.sampleplayer.new

```lua
playdate.sound.sampleplayer.new(path: string): _SamplePlayer
playdate.sound.sampleplayer.new(sample: _Sample): _SamplePlayer
```

Returns a new playdate.sound.sampleplayer object, with the sound data loaded in memory. If the sample can’t be loaded, the function returns nil and a second value containing the error.

### playdate.sound.sampleplayer:copy

```lua
playdate.sound.sampleplayer:copy(): _SamplePlayer
```

Returns a new playdate.sound.sampleplayer with the same sample, volume, and rate as the given sampleplayer.

### playdate.sound.sampleplayer:getLength

```lua
playdate.sound.sampleplayer:getLength(): number
```

Returns the length of the sampleplayer’s sample, in seconds. Length is not scaled by playback rate.

### playdate.sound.sampleplayer:getOffset

```lua
playdate.sound.sampleplayer:getOffset(): number
```

Returns the current offset of the sampleplayer, in seconds. This value is not adjusted for rate.

### playdate.sound.sampleplayer:getRate

```lua
playdate.sound.sampleplayer:getRate(): number
```

Returns the playback rate for the sample.

### playdate.sound.sampleplayer:getSample

```lua
playdate.sound.sampleplayer:getSample(): _Sample
```

Gets the sample to be played.

### playdate.sound.sampleplayer:getVolume

```lua
playdate.sound.sampleplayer:getVolume(): (number, number?)
```

Returns the playback volume for the sampleplayer, a single value for mono sources or a pair of values (left, right) for stereo sources.

### playdate.sound.sampleplayer:isPlaying

```lua
playdate.sound.sampleplayer:isPlaying(): boolean
```

Returns a boolean indicating whether the sample is playing.

### playdate.sound.sampleplayer:play

```lua
playdate.sound.sampleplayer:play(repeatCount: integer, rate: number): nil
```

Starts playing the sample. If *repeatCount* is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with playdate.sound.sampleplayer:stop(). If *rate* is set, the sample will be played at the given rate instead of the rate previous set with playdate.sound.sampleplayer.setRate().

### playdate.sound.sampleplayer:playAt

```lua
playdate.sound.sampleplayer:playAt(when: number, vol: number, rightvol: number, rate: number): nil
```

Schedules the sound for playing at device time *when*. If *vol* is specified, the sample will be played at level *vol* (with optional separate right channel volume *rightvol*), otherwise it plays at the volume set by playdate.sound.sampleplayer.setVolume(). Note that the *when* argument is an offset in the audio device’s time scale, as returned by playdate.sound.getCurrentTime(); it is **not** relative to the current time! If *when* is less than the current audio time, the sample is played immediately. If *rate* is set, the sample will be played at the given rate instead of the rate previously set with playdate.sound.sampleplayer.setRate().
Only one event can be queued at a time. If `playAt()` is called while another event is queued, it will overwrite it with the new values.
The function returns true if the sample was successfully added to the sound channel, otherwise false (i.e., if the channel is full).

### playdate.sound.sampleplayer:setFinishCallback

```lua
playdate.sound.sampleplayer:setFinishCallback(func: function, arg: any): nil
```

Sets a function to be called when playback has completed. The sample object is passed to this function as the first argument, and the optional *arg* argument is passed as the second.

### playdate.sound.sampleplayer:setLoopCallback

```lua
playdate.sound.sampleplayer:setLoopCallback(callback: function, arg: any): nil
```

Sets a function to be called every time the sample loops. The sample object is passed to this function as the first argument, and the optional *arg* argument is passed as the second.

### playdate.sound.sampleplayer:setOffset

```lua
playdate.sound.sampleplayer:setOffset(seconds: number): nil
```

Sets the current offset of the sampleplayer, in seconds. This value is not adjusted for rate.

### playdate.sound.sampleplayer:setPaused

```lua
playdate.sound.sampleplayer:setPaused(flag: boolean): nil
```

Pauses or resumes playback.

### playdate.sound.sampleplayer:setPlayRange

```lua
playdate.sound.sampleplayer:setPlayRange(start: integer, _end: integer): nil
```

Sets the range of the sample to play. *start* and *end* are frame offsets from the beginning of the sample.

### playdate.sound.sampleplayer:setRate

```lua
playdate.sound.sampleplayer:setRate(rate: number): nil
```

Sets the playback rate for the sample. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Sampleplayers can also play samples backwards, by setting a negative rate; note, however, this does not work with ADPCM-encoded files.

### playdate.sound.sampleplayer:setRateMod

```lua
playdate.sound.sampleplayer:setRateMod(signal: _Signal): nil
```

Sets the signal to use as a rate modulator, added to the rate set with playdate.sound.sampleplayer:setRate().  Set to *nil* to clear the modulator.

### playdate.sound.sampleplayer:setSample

```lua
playdate.sound.sampleplayer:setSample(sample: _Sample): nil
```

Sets the sample to be played.

### playdate.sound.sampleplayer:setVolume

```lua
playdate.sound.sampleplayer:setVolume(left: number, right: number): nil
```

Sets the playback volume (0.0 - 1.0) for left and right channels. If the optional *right* argument is omitted, it is the same as *left*. If the sampleplayer is currently playing using the default volume (that is, it wasn’t triggered by `playAt()` with a volume given) it also changes the volume of the playing sample.

### playdate.sound.sampleplayer:stop

```lua
playdate.sound.sampleplayer:stop(): nil
```

Stops playing the sample.
