# playdate.sound.channel

## Functions

### playdate.sound.channel.new

```lua
playdate.sound.channel.new(): _Channel
```

Returns a new channel object and adds it to the global list.

### playdate.sound.channel:addEffect

```lua
playdate.sound.channel:addEffect(effect: _SoundEffect): nil
```

Adds an effect to the channel.

### playdate.sound.channel:addSource

```lua
playdate.sound.channel:addSource(source: _SoundSource): nil
```

Adds a source to the channel. If a source is not assigned to a channel, it plays on the default global channel.

### playdate.sound.channel:getDryLevelSignal

```lua
playdate.sound.channel:getDryLevelSignal(): _Signal
```

Returns a signal that follows the volume of the channel before effects are applied.

### playdate.sound.channel:getVolume

```lua
playdate.sound.channel:getVolume(): number
```

Gets the volume (0.0 - 1.0) for the channel.

### playdate.sound.channel:getWetLevelSignal

```lua
playdate.sound.channel:getWetLevelSignal(): _Signal
```

Returns a signal that follows the volume of the channel after effects are applied.

### playdate.sound.channel:remove

```lua
playdate.sound.channel:remove(): nil
```

Removes the channel from the global list.

### playdate.sound.channel:removeEffect

```lua
playdate.sound.channel:removeEffect(effect: _SoundEffect): nil
```

Removes an effect from the channel.

### playdate.sound.channel:removeSource

```lua
playdate.sound.channel:removeSource(source: _SoundSource): nil
```

Removes a source from the channel.

### playdate.sound.channel:setPan

```lua
playdate.sound.channel:setPan(pan: number): number
```

Sets the pan parameter for the channel. -1 is left, 0 is center, and 1 is right.

### playdate.sound.channel:setPanMod

```lua
playdate.sound.channel:setPanMod(signal: _Signal): nil
```

Sets a signal to automate the pan parameter. Set to *nil* to clear the modulator.

### playdate.sound.channel:setVolume

```lua
playdate.sound.channel:setVolume(volume: number): nil
```

Sets the volume (0.0 - 1.0) for the channel.

### playdate.sound.channel:setVolumeMod

```lua
playdate.sound.channel:setVolumeMod(signal: _Signal): nil
```

Sets a signal to automate the volume parameter. Set to *nil* to clear the modulator.
