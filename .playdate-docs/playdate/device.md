# playdate.device

## Functions

### playdate.deviceDidUnlock

```lua
playdate.deviceDidUnlock(): nil
```

If your game is running on the Playdate when the device is unlocked, this function will be called.

### playdate.deviceWillLock

```lua
playdate.deviceWillLock(): nil
```

If your game is running on the Playdate when the device is locked, this function will be called. Implementing this function allows your game to take special action when the Playdate is locked, e.g., saving state.

### playdate.deviceWillSleep

```lua
playdate.deviceWillSleep(): nil
```

Called before the device goes to low-power sleep mode because of a low battery.

### playdate.gameWillPause

```lua
playdate.gameWillPause(): nil
```

Called before the system pauses the game. (In the current version of Playdate OS, this only happens when the device’s Menu button is pushed.) Implementing these functions allows your game to take special action when it is paused, e.g., updating the menu image.

### playdate.getBatteryPercentage

```lua
playdate.getBatteryPercentage(): integer
```

Returns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 = full.

### playdate.getBatteryVoltage

```lua
playdate.getBatteryVoltage(): number
```

Returns the battery’s current voltage level.

### playdate.getPowerStatus

```lua
playdate.getPowerStatus(): _PowerStatus
```

Returns a table holding booleans with the following keys:
* *charging*: The battery is actively being charged
* *USB*: There is a powered USB cable connected
* *screws*: There is 5V being applied to the corner screws (via the dock, for example)

### playdate.setAutoLockDisabled

```lua
playdate.setAutoLockDisabled(disable: boolean): nil
```

*True* disables the 3 minute auto-lock feature. *False* re-enables it and resets the timer back to 3 minutes.
Auto-lock will automatically be re-enabled when your game terminates.
If disabling auto-lock, developers should look for opportunities to re-enable auto-lock when appropriate. (For example, if your game is an MP3 audio player, auto-lock could be re-enabled when the user pauses the audio.)

## Classes

### _PowerStatus

```lua
---@class _PowerStatus
---@field charging boolean
---@field _USB boolean
---@field screws boolean
```
