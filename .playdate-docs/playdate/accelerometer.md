# playdate.accelerometer

## Functions

### playdate.accelerometerIsRunning

```lua
playdate.accelerometerIsRunning(): boolean
```

Returns true if the accelerometer is currently running.

### playdate.readAccelerometer

```lua
playdate.readAccelerometer(): (number, number, number)
```

If the accelerometer has been turned on with playdate.startAccelerometer(), returns the x, y, and z values from the accelerometer as a list. Positive x points right, positive y points to the bottom of the screen, and positive z points through the screen away from the viewer. For example, with the device held upright this function returns the values (0,1,0). With it flat on its back, it returns (0,0,1).

### playdate.startAccelerometer

```lua
playdate.startAccelerometer(): nil
```

The accelerometer is off by default, to save a bit of power. If you will be using the accelerometer in your game, you’ll first need to call `playdate.startAccelerometer()` then wait for the next update cycle before reading its values. If you won’t be using the accelerometer again for a while, calling `playdate.stopAccelerometer()` will put it back into a low-power idle state.

### playdate.stopAccelerometer

```lua
playdate.stopAccelerometer(): nil
```

Puts the accelerometer into a low-power idle state. (Though, to be honest, the accelerometer draws so little power when it’s running you’d never notice the difference.)
