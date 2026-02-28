# playdate.sound.micinput

## Functions

### playdate.sound.micinput.getLevel

```lua
playdate.sound.micinput.getLevel(): number
```

Returns the current microphone input level, a value from 0.0 (quietest) to 1.0 (loudest).

### playdate.sound.micinput.getSource

```lua
playdate.sound.micinput.getSource(): string
```

Returns the current microphone input source, either "headset" or "device".

### playdate.sound.micinput.recordToSample

```lua
playdate.sound.micinput.recordToSample(buffer: _Sample, completionCallback: fun(sample: _Sample): nil): nil
```

`buffer` should be a Sample created with the following code, with *secondsToRecord* replaced by a number specifying the record duration:
```
local buffer = playdate.sound.sample.new(_secondsToRecord_, playdate.sound.kFormat16bitMono)
```
`completionCallback` is a function called at the end of recording, when the buffer is full. It has one argument, the recorded sample. To override the device’s headset detection and force recording from either the internal mic or a headset mic or line in connected to a headset splitter, first call playdate.sound.micinput.startListening() with the required source. `recordToSample()` returns `true` on success, `false` on error.

### playdate.sound.micinput.startListening

```lua
playdate.sound.micinput.startListening(source: string): (boolean, string)
```

Starts monitoring the microphone input level. The optional *source* argument of "headset" or "device" causes the mic input to record from the given source. If no source is given, it uses the headset detection circuit to determine which source to use. The function returns the pair `true` and a string indicating which source it’s recording from on success, or `false` on error.

### playdate.sound.micinput.stopListening

```lua
playdate.sound.micinput.stopListening(): nil
```

Stops monitoring the microphone input level.

### playdate.sound.micinput.stopRecording

```lua
playdate.sound.micinput.stopRecording(): nil
```

Stops a sample recording started with recordToSample, if it hasn’t already reached the end of the buffer. The recording’s completion callback is called immediately.
