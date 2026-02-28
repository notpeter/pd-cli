# playdate.sound.sample

## Functions

### playdate.sound.sample.new

```lua
playdate.sound.sample.new(path: string): _Sample
playdate.sound.sample.new(seconds: number, format: integer): _Sample
```

Returns a new playdate.sound.sample object, with the sound data loaded in memory. If the sample can’t be loaded, the function returns nil and a second value containing the error.

### playdate.sound.sample:decompress

```lua
playdate.sound.sample:decompress(): (boolean, string)
```

If the sample is ADPCM compressed, decompresses the sample data to 16-bit PCM data. This increases the sample’s memory footprint by 4x and does not affect the quality in any way, but it is necessary if you want to use the sample in a synth or play the file backwards. Returns `true` if successful, or `false` and an error message as a second return value if decompression failed.

### playdate.sound.sample:getFormat

```lua
playdate.sound.sample:getFormat(): integer
```

Returns the format of the sample, one of
* *playdate.sound.kFormat8bitMono*
* *playdate.sound.kFormat8bitStereo*
* *playdate.sound.kFormat16bitMono*
* *playdate.sound.kFormat16bitStereo*

### playdate.sound.sample:getLength

```lua
playdate.sound.sample:getLength(): (number, number)
```

Returns two values, the length of the available sample data and the size of the allocated buffer. Both values are measured in seconds. For a sample loaded from disk, these will be the same; for a sample used for recording, the available data may be less than the allocated size.

### playdate.sound.sample:getSampleRate

```lua
playdate.sound.sample:getSampleRate(): integer
```

Returns the sample rate as an integer, such as 44100 or 22050.

### playdate.sound.sample:getSubsample

```lua
playdate.sound.sample:getSubsample(startOffset: integer, endOffset: integer): _Sample
```

Returns a new subsample containing a subrange of the given sample. Offset values are in frames, not bytes.

### playdate.sound.sample:load

```lua
playdate.sound.sample:load(path: string): boolean
```

Loads the sound data from the file at *path* into an existing sample buffer. If there is no file at *path*, the function returns nil.

### playdate.sound.sample:play

```lua
playdate.sound.sample:play(repeatCount: integer, rate: number): nil
```

Convenience function: Creates a new sampleplayer for the sample and passes the function arguments to its play function.

### playdate.sound.sample:playAt

```lua
playdate.sound.sample:playAt(when: number, vol: number, rightvol: number, rate: number): nil
```

Convenience function: Creates a new sampleplayer for the sample and passes the function arguments to its playAt function.

### playdate.sound.sample:save

```lua
playdate.sound.sample:save(filename: string): nil
```

Saves the sample to the given file. If `filename` has a `.wav` extension it will be saved in WAV format (and be unreadable by the Playdate sound functions), otherwise it will be saved in the Playdate pda format.
