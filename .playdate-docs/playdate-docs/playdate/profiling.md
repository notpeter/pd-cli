# playdate.profiling

## Functions

### playdate.getStats

```lua
playdate.getStats(): _SystemStats
```

Returns a table containing percentages of time spent in each system task over the last interval, if more than zero. Possible keys are
* `kernel`
* `serial`
* `game`
* `GC`
* `wifi`
* `audio`
* `trace`
* `idle`
`playdate.getStats()` only functions on a Playdate device. In the Simulator, this function returns `nil`.

### playdate.setStatsInterval

```lua
playdate.setStatsInterval(seconds: number): nil
```

`setStatsInterval()` sets the length of time for each sample frame of runtime stats. Set *seconds* to zero to disable stats collection.

## Classes

### _SystemStats

```lua
---@class _SystemStats
---@field audio number
---@field game number
---@field idle number
---@field kernel number
---@field serial? number
---@field trace? number
---@field wifi? number
---@field GC? number
```
