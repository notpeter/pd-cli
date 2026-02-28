# playdate.sound.envelope

## Functions

### playdate.sound.envelope.new

```lua
playdate.sound.envelope.new(attack: number, decay: number, sustain: number, release: number): _Envelope
```

Creates a new envelope with the given (optional) parameters.

### playdate.sound.envelope:getValue

```lua
playdate.sound.envelope:getValue(): number
```

Returns the current signal value of the envelope.

### playdate.sound.envelope:setAttack

```lua
playdate.sound.envelope:setAttack(attack: number): nil
```

Sets the envelope attack time to *attack*, in seconds.

### playdate.sound.envelope:setCurvature

```lua
playdate.sound.envelope:setCurvature(amount: number): nil
```

Smoothly changes the envelope’s shape from linear (amount=0) to exponential (amount=1).

### playdate.sound.envelope:setDecay

```lua
playdate.sound.envelope:setDecay(decay: number): nil
```

Sets the envelope decay time to *decay*, in seconds.

### playdate.sound.envelope:setGlobal

```lua
playdate.sound.envelope:setGlobal(flag: boolean): nil
```

If an envelope is marked global, it is continuously updated whether or not it’s attached to any source.

### playdate.sound.envelope:setLegato

```lua
playdate.sound.envelope:setLegato(flag: boolean): nil
```

Sets whether to use legato phrasing for the envelope. If the legato flag is set, when the envelope is re-triggered before it’s released, it remains in the sustain phase instead of jumping back to the attack phase.

### playdate.sound.envelope:setOffset

```lua
playdate.sound.envelope:setOffset(offset: number): nil
```

Sets the offset value for the envelope. The transformed envelope has an initial value of *offset* and a maximum (minimum if *scale* is negative) of *offset* + *scale*.

### playdate.sound.envelope:setRateScaling

```lua
playdate.sound.envelope:setRateScaling(scaling: number, start: number, _end: number): nil
```

Scales the envelope rate according to the played note. For notes below `start`, the envelope’s set rate is used; for notes above `end` envelope rates are scaled by the `scaling` parameter. Between the two notes the scaling factor is interpolated from 1.0 to `scaling`. `start` and `end` are either MIDI note numbers or names like "C4". If omitted, the default range is C1 (36) to C5 (84).

### playdate.sound.envelope:setRelease

```lua
playdate.sound.envelope:setRelease(release: number): nil
```

Sets the envelope release time to *release*, in seconds.

### playdate.sound.envelope:setRetrigger

```lua
playdate.sound.envelope:setRetrigger(flag: boolean): nil
```

If retrigger is on, the envelope always starts from 0 when a note starts playing, instead of the current value if it’s active.

### playdate.sound.envelope:setScale

```lua
playdate.sound.envelope:setScale(scale: integer): nil
```

Sets the scale value for the envelope. The transformed envelope has an initial value of *offset* and a maximum (minimum if *scale* is negative) of *offset* + *scale*.

### playdate.sound.envelope:setSustain

```lua
playdate.sound.envelope:setSustain(sustain: number): nil
```

Sets the envelope sustain level to *sustain*, as a proportion of the maximum. For example, if the sustain level is 0.5, the signal value rises to its full value over the attack phase of the envelope, then drops to half its maximum over the decay phase, and remains there while the envelope is active.

### playdate.sound.envelope:setVelocitySensitivity

```lua
playdate.sound.envelope:setVelocitySensitivity(amount: number): nil
```

Changes the amount by which note velocity scales output level. At the default value of 1, output is proportional to velocity; at 0 velocity has no effect on output level.

### playdate.sound.envelope:trigger

```lua
playdate.sound.envelope:trigger(velocity: number, length: number): nil
```

Triggers the envelope at the given *velocity*. If a *length* parameter is given, the envelope moves to the release phase after the given time. Otherwise, the envelope is held in the sustain phase until the trigger function is called again with *velocity* equal to zero.
