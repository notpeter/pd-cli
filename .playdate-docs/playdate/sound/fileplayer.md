# playdate.sound.fileplayer

## Functions

### playdate.sound.fileplayer.new

```lua
playdate.sound.fileplayer.new(buffersize: number): _FilePlayer
playdate.sound.fileplayer.new(path: string, buffersize: number): _FilePlayer
```

Returns a fileplayer object, which can stream samples from disk. The file to play is set with the playdate.sound.fileplayer:load() function.
If given, *buffersize* specifies the size in seconds of the fileplayer’s data buffer. A shorter value reduces the latency of a playdate.sound.fileplayer:setOffset() call, but increases the chance of a buffer underrun.

### playdate.sound.fileplayer:didUnderrun

```lua
playdate.sound.fileplayer:didUnderrun(): boolean
```

Returns the fileplayer’s underrun flag, indicating that the player ran out of data. This can be checked in the finish callback function to check for an underrun error.

### playdate.sound.fileplayer:getLength

```lua
playdate.sound.fileplayer:getLength(): number
```

Returns the length, in seconds, of the audio file.

### playdate.sound.fileplayer:getOffset

```lua
playdate.sound.fileplayer:getOffset(): number
```

Returns the current offset of the fileplayer, in seconds. This value is not adjusted for rate.

### playdate.sound.fileplayer:getRate

```lua
playdate.sound.fileplayer:getRate(): number
```

Returns the playback rate for the file. as set with `setRate()`.

### playdate.sound.fileplayer:getVolume

```lua
playdate.sound.fileplayer:getVolume(): (number, number?)
```

Returns the current volume for the fileplayer, a single value for mono sources or a pair of values (left, right) for stereo sources.

### playdate.sound.fileplayer:isPlaying

```lua
playdate.sound.fileplayer:isPlaying(): boolean
```

Returns a boolean indicating whether the fileplayer is playing.

### playdate.sound.fileplayer:load

```lua
playdate.sound.fileplayer:load(path: string): nil
```

Instructs the fileplayer to load the file at *path* when play() is called on it. The fileplayer must not be playing when this function is called. The fileplayer’s play offset is reset to the beginning of the file, and its loop range is cleared.

### playdate.sound.fileplayer:pause

```lua
playdate.sound.fileplayer:pause(): nil
```

Stops playing the file. A subsequent play() call resumes playback from where it was paused.

### playdate.sound.fileplayer:play

```lua
playdate.sound.fileplayer:play(repeatCount: integer): (boolean, string?)
```

Opens and starts playing the file, first creating and filling a 1/4 second playback buffer if a buffer size hasn’t been set yet.
If repeatCount is set, playback repeats when it reaches the end of the file or the end of the loop range if one is set. After the loop has run *repeatCount* times, it continues playing to the end of the file. A *repeatCount* of zero loops endlessly. If repeatCount is not set, the file plays once.
The function returns true if the file was successfully opened and the fileplayer added to the sound channel, otherwise false and a string describing the error.

### playdate.sound.fileplayer:setBufferSize

```lua
playdate.sound.fileplayer:setBufferSize(seconds: number): nil
```

Sets the buffer size for the fileplayer, in seconds. Larger buffers protect against buffer underruns, but consume more memory. Calling this function also fills the output buffer if a source file has been set. On success, the function returns *true*; otherwise it returns *false* and a string describing the error.

### playdate.sound.fileplayer:setFinishCallback

```lua
playdate.sound.fileplayer:setFinishCallback(func: fun(self: _FilePlayer, arg?: any), arg: any): nil
```

Sets a function to be called when playback has completed. The fileplayer is passed as the first argument to *func*. The optional argument *arg* is passed as the second.

### playdate.sound.fileplayer:setLoopCallback

```lua
playdate.sound.fileplayer:setLoopCallback(callback: fun(self: _FilePlayer, arg?: any), arg: any): nil
```

Sets a function to be called every time the fileplayer loops. The fileplayer object is passed to this function as the first argument, and *arg* as the second.
The fileplayer:play([repeatCount]) call needs to be invoked with a *repeatCount* value of 0 (infinite looping), or 2 or greater in order for the loop callback to be invoked.

### playdate.sound.fileplayer:setLoopRange

```lua
playdate.sound.fileplayer:setLoopRange(start: number, _end: number, loopCallback: fun(arg?: any): nil, arg: any): nil
```

Provides a way to loop a portion of an audio file. In the following code:
```
local fp = playdate.sound.fileplayer.new( "myaudiofile" )
fp:setLoopRange( 10, 20 )
fp:play( 3 )
```
…the fileplayer will start playing from the beginning of the audio file, loop the 10-20 second range three times, and then stop playing.
*start* and *end* are specified in seconds. If *end* is omitted, the end of the file is used. If the function *loopCallback* is provided, it is called every time the player loops, with the fileplayer as the first argument and the optional *arg* argument as the second.
The fileplayer:play([repeatCount]) call needs to be invoked with a *repeatCount* value of 0 (infinite looping), or 2 or greater in order for the looping action to happen.

### playdate.sound.fileplayer:setOffset

```lua
playdate.sound.fileplayer:setOffset(seconds: number): nil
```

Sets the current offset of the fileplayer, in seconds. This value is not adjusted for rate.

### playdate.sound.fileplayer:setRate

```lua
playdate.sound.fileplayer:setRate(rate: number): nil
```

Sets the playback rate for the file. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Unlike sampleplayers, fileplayers can’t play in reverse (i.e., rate < 0).

### playdate.sound.fileplayer:setRateMod

```lua
playdate.sound.fileplayer:setRateMod(signal: _Signal): nil
```

Sets the signal to use as a rate modulator, added to the rate set with playdate.sound.fileplayer:setRate().  Set to *nil* to clear the modulator.

### playdate.sound.fileplayer:setStopOnUnderrun

```lua
playdate.sound.fileplayer:setStopOnUnderrun(flag: boolean): nil
```

By default, if the fileplayer runs out of data it does not stop playback but instead restarts (after an audible stutter) as soon as data becomes available. Setting the flag to *true* changes this behavior so that it stops playback and calls the fileplayer’s finish callback, if set.

### playdate.sound.fileplayer:setVolume

```lua
playdate.sound.fileplayer:setVolume(left: number, right: number, fadeSeconds: number, fadeCallback: fun(self: _FilePlayer, arg?: any), arg: any): nil
```

Sets the playback volume (0.0 - 1.0). If a single value is passed in, both left side and right side volume are set to the given value. If two values are given, volumes are set separately. The optional *fadeSeconds* specifies the time it takes to fade from the current volume to the specified volume, in seconds. If the function *fadeCallback* is given, it is called when the volume fade has completed. The fileplayer object is passed as the first argument to the callback, and the optional *arg* argument is passed as the second.

### playdate.sound.fileplayer:stop

```lua
playdate.sound.fileplayer:stop(): nil
```

Stops playing the file, resets the playback offset to zero, and calls the finish callback.
