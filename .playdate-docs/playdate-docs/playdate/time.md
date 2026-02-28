# playdate.time

## Functions

### _FrameTimer:timerEndedCallback

```lua
_FrameTimer:timerEndedCallback(...: any): nil
```

A Function of the form *function(timer)* or *function(...)* where "..." corresponds to the values in the table assigned to *timerEndedArgs*. Called when the timer has completed.

### _FrameTimer:updateCallback

```lua
_FrameTimer:updateCallback(...: any): nil
```

A function to be called on every frame update. If the frame timer was created with arguments, those will be passed as arguments to the function provided. Otherwise, the timer is passed as the single argument.

### _Timer:timerEndedCallback

```lua
_Timer:timerEndedCallback(...: any): nil
```

A Function of the form *function(timer)* or *function(...)* where "..." corresponds to the values in the table assigned to *timerEndedArgs*. Called when the timer has completed.

### _Timer:updateCallback

```lua
_Timer:updateCallback(...: any): nil
```

A callback function that will be called on every frame (every time *timer.updateAll()* is called). If the timer was created with arguments, those will be passed as arguments to the function provided. Otherwise, the timer is passed as the single argument.

### playdate.GMTTimeFromEpoch

```lua
playdate.GMTTimeFromEpoch(seconds: integer, milliseconds: integer): _DateTime
```

Converts the epoch to a GMT date and time table, in the same format as the table returned by playdate.getTime().

### playdate.epochFromGMTTime

```lua
playdate.epochFromGMTTime(time: _DateTime): (integer, integer)
```

Returns the number of seconds and milliseconds between midnight (hour 0), January 1 2000 UTC and *time*, specified in GMT time, as a list: *(seconds, milliseconds)*.
*time* should be a table of the same format as the one returned by playdate.getTime().

### playdate.epochFromTime

```lua
playdate.epochFromTime(time: _DateTime): (integer, integer)
```

Returns the number of seconds and milliseconds between midnight (hour 0), January 1 2000 UTC and *time*, specified in local time, as a list: *(seconds, milliseconds)*.
*time* should be a table of the same format as the one returned by playdate.getTime().

### playdate.file.modtime

```lua
playdate.file.modtime(path: string): _ModTime
```

Returns the modification date/time of the file at the given path, as a table with keys:
* *year*: 4-digit year (until 10,000 AD)
* *month*: month of the year, where 1 is January and 12 is December
* *day*: day of the month, 1 - 31
* *hour*: 0 - 23
* *minute*: 0 - 59
* *second*: 0 - 59 (or 60 on a leap second)

### playdate.frameTimer.allTimers

```lua
playdate.frameTimer.allTimers(): _FrameTimer[]
```

Returns an array listing all running frameTimers.
Note the "." syntax rather than ":". This is a class method, not an instance method.

### playdate.frameTimer.new

```lua
playdate.frameTimer.new(duration: integer, callback: function, ...: any): _FrameTimer
playdate.frameTimer.new(duration: integer, startValue: number, endValue: number, easingFunction: function): _FrameTimer
```

Returns a new playdate.frameTimer that will run for *duration* frames. *callback* is a function closure that will be called when the timer is complete.
Accepts a variable number of arguments that will be passed to the callback function when it is called. If arguments are not provided, the timer itself will be passed to the callback instead.
By default, frame timers start upon instantiation. To modify the behavior of a frame timer, see common frame timer methods and properties.

### playdate.frameTimer.performAfterDelay

```lua
playdate.frameTimer.performAfterDelay(delay: integer, callback: function, ...: any): nil
```

Performs the function *callback* after the *delay* number of frames. Accepts a variable number of arguments that will be passed to the callback function when it is called. If arguments are not provided, the timer itself will be passed to the callback instead.

### playdate.frameTimer.updateTimers

```lua
playdate.frameTimer.updateTimers(): nil
```

This should be called from the main playdate.update() loop to drive the frame timers.

### playdate.frameTimer:pause

```lua
playdate.frameTimer:pause(): nil
```

Pauses a timer.

### playdate.frameTimer:remove

```lua
playdate.frameTimer:remove(): nil
```

Removes this timer from the list of timers. This happens automatically when a non-repeating timer reaches it’s end, but you can use this method to dispose of timers manually.

### playdate.frameTimer:reset

```lua
playdate.frameTimer:reset(): nil
```

Resets a timer to its initial values.

### playdate.frameTimer:start

```lua
playdate.frameTimer:start(): nil
```

Resumes a timer. There is no need to call :start() on a newly-instantiated frame timer: frame timers start automatically.

### playdate.getCurrentTimeMilliseconds

```lua
playdate.getCurrentTimeMilliseconds(): integer
```

Returns the number of milliseconds the game has been *active* since launched.

### playdate.getElapsedTime

```lua
playdate.getElapsedTime(): number
```

Returns the number of seconds since `playdate.resetElapsedTime()` was called. The value is a floating-point number with microsecond accuracy.

### playdate.getGMTTime

```lua
playdate.getGMTTime(): _DateTime
```

Returns a table in the same format as playdate.getTime(), but in GMT rather than local time.

### playdate.getSecondsSinceEpoch

```lua
playdate.getSecondsSinceEpoch(): (integer, integer)
```

Returns the number of seconds and milliseconds elapsed since midnight (hour 0), January 1 2000 UTC, as a list: *(seconds, milliseconds)*. This function is suitable for seeding the random number generator:
```
math.randomseed(playdate.getSecondsSinceEpoch())
```

### playdate.getServerTime

```lua
playdate.getServerTime(callback: fun(time?: string, error?: string)): nil
```

Queries the Playdate server for the current time, in seconds elapsed since midnight (hour 0), January 1 2000 UTC. This provides games with a reliable clock source, since the internal clock can be set by the user. The function is asynchronous, returning the server time to a callback function passed in. The callback function is given two arguments: the time (as a string, to avoid 32-bit rollover) if the query was successful, otherwise nil and an error string.
```
playdate.getServerTime(function(time, error)
    if time ~= nil then print("server time: "..time)
    else print("server error: "..error)
    end
end)
```

### playdate.getTime

```lua
playdate.getTime(): _DateTime
```

Returns a table with values for the local time, accessible via the following keys:
* *year*: 4-digit year (until 10,000 AD)
* *month*: month of the year, where 1 is January and 12 is December
* *day*: day of the month, 1 - 31
* *weekday*: day of the week, where 1 is Monday and 7 is Sunday
* *hour*: 0 - 23
* *minute*: 0 - 59
* *second*: 0 - 59 (or 60 on a leap second)
* *millisecond*: 0 - 999

### playdate.graphics.animator:valueAtTime

```lua
playdate.graphics.animator:valueAtTime(time: number): (number|_Point)
```

Returns the value of the animation at the given number of milliseconds after the start time. The value will be either a number or a playdate.geometry.point, depending on the type of animator.

### playdate.network.http:setConnectTimeout

```lua
playdate.network.http:setConnectTimeout(seconds: integer): nil
```

Sets the length of time (in seconds) to wait for the connection to the server to be made.

### playdate.network.http:setReadTimeout

```lua
playdate.network.http:setReadTimeout(seconds: number): nil
```

Sets the length of time, in seconds, `playdate.network.http:read()` will wait for incoming data before returning. The default value is one second.

### playdate.network.tcp:setConnectTimeout

```lua
playdate.network.tcp:setConnectTimeout(seconds: integer): nil
```

Sets the length of time (in seconds) to wait for the connection to the server to be made.

### playdate.network.tcp:setReadTimeout

```lua
playdate.network.tcp:setReadTimeout(seconds: number): nil
```

Sets the length of time, in seconds, `playdate.network.tcp:read()` will wait for incoming data before returning. The default value is one second.

### playdate.resetElapsedTime

```lua
playdate.resetElapsedTime(): nil
```

Resets the high-resolution timer.

### playdate.setMinimumGCTime

```lua
playdate.setMinimumGCTime(ms: integer): nil
```

Force the Lua garbage collector to run for at least *ms* milliseconds every frame, so that garbage doesn’t pile up and cause the game to run out of memory and stall in emergency garbage collection. The default value is 1 millisecond.
If your game isn’t generating a lot of garbage, it might be advantageous to set a smaller minimum GC time, granting more CPU bandwidth to your game.

### playdate.shouldDisplay24HourTime

```lua
playdate.shouldDisplay24HourTime(): boolean
```

Returns true if the user has set the 24-Hour Time preference in the Settings program.

### playdate.sound.getCurrentTime

```lua
playdate.sound.getCurrentTime(): number
```

Returns the current time, in seconds, as measured by the audio device. The audio device uses its own time base in order to provide accurate timing.
Equivalent to `playdate->sound->getCurrentTime()` in the C API.

### playdate.sound.resetTime

```lua
playdate.sound.resetTime(): nil
```

Resets the audio output device time counter.

### playdate.timeFromEpoch

```lua
playdate.timeFromEpoch(seconds: integer, milliseconds: integer): _DateTime
```

Converts the epoch to a local date and time table, in the same format as the table returned by playdate.getTime().

### playdate.timer.allTimers

```lua
playdate.timer.allTimers(): _Timer[]
```

Returns an array listing all running timers.
Note the "." syntax rather than ":". This is a class method, not an instance method.

### playdate.timer.keyRepeatTimer

```lua
playdate.timer.keyRepeatTimer(callback: function, ...: any): _Timer
```

Calls `keyRepeatTimerWithDelay()` below with standard values of *delayAfterInitialFiring* = 300 and *delayAfterSecondFiring* = 100.

### playdate.timer.keyRepeatTimerWithDelay

```lua
playdate.timer.keyRepeatTimerWithDelay(delayAfterInitialFiring: integer, delayAfterSecondFiring: integer, callback: function, ...: any): _Timer
```

returns a timer that fires at key-repeat intervals. The function *callback* will be called immediately, then again after *delayAfterInitialFiring* milliseconds, then repeatedly at *delayAfterSecondFiring* millisecond intervals.

### playdate.timer.new

```lua
playdate.timer.new(duration: integer, callback: function, ...: any): _Timer
playdate.timer.new(duration: integer, startValue: number, endValue: number, easingFunction: function): _Timer
```

Returns a new playdate.timer that will run for *duration* milliseconds. *callback* is a function closure that will be called when the timer is complete.
Accepts a variable number of arguments that will be passed to the callback function when it is called. If arguments are not provided, the timer itself will be passed to the callback instead.
By default, timers start upon instantiation. To modify the behavior of a timer, see common timer methods and properties.

### playdate.timer.performAfterDelay

```lua
playdate.timer.performAfterDelay(delay: integer, callback: function, ...: any): nil
```

Performs the function *callback* after *delay* milliseconds. Accepts a variable number of arguments that will be passed to the callback function when it is called. If arguments are not provided, the timer itself will be passed to the callback instead.

### playdate.timer.updateTimers

```lua
playdate.timer.updateTimers(): nil
```

This should be called from the main playdate.update() loop to drive the timers.

### playdate.timer:pause

```lua
playdate.timer:pause(): nil
```

Pauses a timer. (There is no need to call :start() on a newly-instantiated timer: timers start automatically.)

### playdate.timer:remove

```lua
playdate.timer:remove(): nil
```

Removes this timer from the list of timers. This happens automatically when a non-repeating timer reaches its end, but you can use this method to dispose of timers manually.
Note that timers do not actually get removed until the next invocation of playdate.timer.updateTimers().

### playdate.timer:reset

```lua
playdate.timer:reset(): nil
```

Resets a timer to its initial values.

### playdate.timer:start

```lua
playdate.timer:start(): nil
```

Resumes a previously paused timer. There is no need to call :start() on a newly-instantiated timer: timers start automatically.

### _Timer:__tostring

```lua
_Timer:__tostring(): string
```

## Classes

### _DateTime

```lua
---@class _DateTime
---@field year integer
---@field month integer
---@field day integer
---@field weekday integer
---@field hour integer
---@field minute integer
---@field second integer
---@field millisecond integer
```

### _ModTime

```lua
---@class _ModTime
---@field year integer
---@field month integer
---@field day integer
---@field hour integer
---@field minute integer
---@field second integer
```
