# profiling

## Functions

### playdate.setCollectsGarbage

```lua
playdate.setCollectsGarbage(flag: boolean): nil
```

If *flag* is false, automatic garbage collection is disabled and the game should manually collect garbage with Lua’s `collectgarbage()` function.

### playdate.setGCScaling

```lua
playdate.setGCScaling(min: number, max: number): nil
```

When the amount of used memory is less than `min` (scaled from 0-1, as a percentage of total system memory), the system will only run the collector for the minimum GC time, as set by playdate.setGCScaling(), every frame. If the used memory is more than `max`, the system will spend all free time running the collector. Between the two, the time used by the garbage collector is scaled proportionally.
For example, if the scaling is set to a min of 0.4 and max of 0.7, and memory is half full, the collector will run for the minimum GC time plus 1/3 of whatever time is left before the next frame (because (0.5 - 0.4) / (0.7 - 0.4) = 1/3).
The default behavior is a scaling of `(0.0, 1.0)`. If set to `(0.0, 0.0)`, the system will use all available extra time each frame running GC.

### sample

```lua
sample(name: string, _function: function): nil
```

Suspect some code is running hot? Wrap it in an anonymous function and pass it to `sample()` like so:
```
sample("name of this sample", function()
        -- nested for loops, lots of table creation, member access...
end)
```
By moving around where you start and end the anonymous function in your code, you can get a better idea of where the problem lies.
Multiple code paths can be sampled at once by using different names for each sample.
You must import *CoreLibs/utilities/sampler* to use this function.

### where

```lua
where(): string
```

Returns a single-line stack trace as a string. For example:
```
main.lua:10 foo() < main.lua:18 (from C)
```
Use `print(where())` to see this trace written to the console.
You must import *CoreLibs/utilities/where* to use this function.
