# playdate.input

## Functions

### playdate.AButtonDown

```lua
playdate.AButtonDown(): nil
```

Called immediately after the player presses the A Button.

### playdate.AButtonHeld

```lua
playdate.AButtonHeld(): nil
```

Called after the A Button is held down for one second. This can be used for secondary actions (e.g., displaying a game world map, changing weapons).

### playdate.AButtonUp

```lua
playdate.AButtonUp(): nil
```

Called immediately after the player releases the A Button.

### playdate.BButtonDown

```lua
playdate.BButtonDown(): nil
```

Called immediately after the player presses the B Button.

### playdate.BButtonHeld

```lua
playdate.BButtonHeld(): nil
```

Called after the B Button is held down for one second. This can be used for secondary actions (e.g., displaying a game world map, changing weapons).

### playdate.BButtonUp

```lua
playdate.BButtonUp(): nil
```

Called immediately after the player releases the B Button.

### playdate.buttonIsPressed

```lua
playdate.buttonIsPressed(button: (integer|string)): boolean
```

Returns true if *button* is currently being pressed.
*button* should be one of the constants:
* *playdate.kButtonA*
* *playdate.kButtonB*
* *playdate.kButtonUp*
* *playdate.kButtonDown*
* *playdate.kButtonLeft*
* *playdate.kButtonRight*
Or one of the strings "a", "b", "up", "down", "left", "right".

### playdate.buttonJustPressed

```lua
playdate.buttonJustPressed(button: (integer|string)): boolean
```

Returns true for *just one update cycle* if *button* was pressed. `buttonJustPressed` will not return true again until the button is released and pressed again. This is useful for, say, a player "jump" action, so the jump action is taken only once and not on every single update.
*button* should be one of the constants listed in playdate.buttonIsPressed()

### playdate.buttonJustReleased

```lua
playdate.buttonJustReleased(button: (integer|string)): boolean
```

Returns true for *just one update cycle* if *button* was released. `buttonJustReleased` will not return true again until the button is pressed and released again.
*button* should be one of the constants listed in playdate.buttonIsPressed()

### playdate.crankDocked

```lua
playdate.crankDocked(): nil
```

This function, if defined, is called when the crank is docked.

### playdate.crankUndocked

```lua
playdate.crankUndocked(): nil
```

This function, if defined, is called when the crank is undocked.

### playdate.cranked

```lua
playdate.cranked(change: number, acceleratedChange: number): nil
```

For playdate.cranked(), *change* is the angle change in degrees. *acceleratedChange* is *change* multiplied by a value that increases as the crank moves faster, similar to the way mouse acceleration works. Negative values are anti-clockwise.

### playdate.downButtonDown

```lua
playdate.downButtonDown(): nil
```

Called immediately after the player presses the down direction on the d-pad.

### playdate.downButtonUp

```lua
playdate.downButtonUp(): nil
```

Called immediately after the player releases the down direction on the d-pad.

### playdate.getButtonState

```lua
playdate.getButtonState(): (integer, integer, integer)
```

Returns the above data in one call, with multiple return values (*current*, *pressed*, *released*) containing bitmasks indicating which buttons are currently down, and which were pressed and released since the last update. For example, if the d-pad left button and the A button are both down, the *current* value will be (*playdate.kButtonA*|*playdate.kButtonLeft*).

### playdate.getCrankChange

```lua
playdate.getCrankChange(): (number, number)
```

Returns two values, *change* and *acceleratedChange*. *change* represents the angle change (in degrees) of the crank since the last time this function (or the playdate.cranked() callback) was called. Negative values are anti-clockwise. *acceleratedChange* is change multiplied by a value that increases as the crank moves faster, similar to the way mouse acceleration works.
```
local change, acceleratedChange = playdate.getCrankChange()
```

### playdate.getCrankPosition

```lua
playdate.getCrankPosition(): number
```

Returns the absolute position of the crank (in degrees). Zero is pointing straight up parallel to the device. Turning the crank clockwise (when looking at the right edge of an upright device) increases the angle, up to a maximum value 359.9999. The value then resets back to zero as the crank continues its rotation.
```
local crankPosition = playdate.getCrankPosition()
```

### playdate.getCrankTicks

```lua
playdate.getCrankTicks(ticksPerRevolution: number): number
```

Returns the number of "ticks" — whose frequency is defined by the value of *ticksPerRevolution*  — the crank has turned through since the last time this function was called. Tick boundaries are set at absolute positions along the crank’s rotation. Ticks can be positive or negative, depending upon the direction of rotation.
For example, say you have a movie player and you want your movie to advance 6 frames for every one revolution of the crank. Calling `playdate.getCrankTicks(6)` during each update will give you a return value of 1 as the crank turns past each 60 degree increment. (Since we passed in a 6, each tick represents 360 ÷ 6 = 60 degrees.) So `getCrankTicks(6)` will return a 1 as the crank turns past the 0 degree absolute position, the 60 degree absolute position, and so on for the 120, 180, 240, and 300 degree positions. Otherwise, 0 will be returned. (-1 will be returned if the crank moves past one of these mentioned positions while going in a backward direction.)
You must import *CoreLibs/crank* to use `getCrankTicks()`.
```
import "CoreLibs/crank"
local ticksPerRevolution = 6
function playdate.update()
    local crankTicks = playdate.getCrankTicks(ticksPerRevolution)
    if crankTicks == 1 then
        print("Forward tick")
    elseif crankTicks == -1 then
        print("Backward tick")
    end
end
```

### playdate.isCrankDocked

```lua
playdate.isCrankDocked(): boolean
```

Returns a boolean indicating whether or not the crank is folded into the unit.
If your game requires the crank and `:isCrankDocked()` is true, you can use a crank alert to notify the user that the crank should be extended.

### playdate.leftButtonDown

```lua
playdate.leftButtonDown(): nil
```

Called immediately after the player presses the left direction on the d-pad.

### playdate.leftButtonUp

```lua
playdate.leftButtonUp(): nil
```

Called immediately after the player releases the left direction on the d-pad.

### playdate.rightButtonDown

```lua
playdate.rightButtonDown(): nil
```

Called immediately after the player presses the right direction on the d-pad.

### playdate.rightButtonUp

```lua
playdate.rightButtonUp(): nil
```

Called immediately after the player releases the right direction on the d-pad.

### playdate.setButtonQueueSize

```lua
playdate.setButtonQueueSize(size: integer): nil
```

When set, button up/down events on the D pad and the A and B buttons are added to a list instead of simply polled at the beginning of a frame, allowing the code to handle multiple taps on a given button in a single frame. At the default 30 FPS, a queue size of 5 should be adequate. At lower frame rates/longer frame times, the queue size should be extended until all button presses are caught. Additionally, when the button queue is enabled the button callbacks listed below are passed the event time as an argument.

### playdate.setCrankSoundsDisabled

```lua
playdate.setCrankSoundsDisabled(disable: boolean): nil
```

*True* disables the default crank docking/undocking sound effects. *False* re-enables them. Useful if the crank sounds seem out-of-place in your game.
When your game terminates, crank sounds will automatically be re-enabled.

### playdate.ui.crankIndicator:draw

```lua
playdate.ui.crankIndicator:draw(xOffset: integer, yOffset: integer): nil
```

Draws the next frame of the crank indicator animation, and is typically invoked in the `playdate.update()` callback. *xOffset* and *yOffset* can be used to alter the position of the indicator by a specified number of pixels if desired. To stop drawing the crank indicator, simply stop calling `:draw()` in `playdate.update()`.
Note that if sprites are being used, this call should usually happen after playdate.graphics.sprite.update().

### playdate.ui.crankIndicator:getBounds

```lua
playdate.ui.crankIndicator:getBounds(xOffset: integer, yOffset: integer): (integer, integer, integer, integer)
```

Returns *x*, *y*, *width*, *height* representing the bounds that the crank indicator draws within. If necessary, this rect could be passed into playdate.graphics.sprite.addDirtyRect(), or used to manually draw over the indicator image drawn by playdate.ui.crankIndicator:draw() when you want to stop showing the crank indicator.

### playdate.ui.crankIndicator:resetAnimation

```lua
playdate.ui.crankIndicator:resetAnimation(): nil
```

Resets the crank animation to the beginning of its sequence.

### playdate.upButtonDown

```lua
playdate.upButtonDown(): nil
```

Called immediately after the player presses the up direction on the d-pad.

### playdate.upButtonUp

```lua
playdate.upButtonUp(): nil
```

Called immediately after the player releases the up direction on the d-pad.

### playdate.ui.crankIndicator:start

```lua
playdate.ui.crankIndicator:start(): nil
```

### playdate.ui.crankIndicator:update

```lua
playdate.ui.crankIndicator:update(): nil
```
