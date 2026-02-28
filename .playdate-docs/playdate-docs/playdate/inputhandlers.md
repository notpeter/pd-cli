# playdate.inputhandlers

## Functions

### playdate.inputHandlers.pop

```lua
playdate.inputHandlers.pop(): nil
```

Pops the last input handler off of the stack.

### playdate.inputHandlers.push

```lua
playdate.inputHandlers.push(handler: _InputHandler, masksPreviousHandlers: boolean): nil
```

Pushes a new input handler onto the stack.
* *handler:* A table containing one or more custom input functions.
* *masksPreviousHandlers:* If true, input functions not defined in *handler* will not be called. If missing or false, the previously-pushed input handler tables will be searched for input functions missing from *handler*, cascading down to the default `playdate` table.

## Classes

### _InputHandler

```lua
---@class _InputHandler
---@field AButtonDown? fun(): nil
---@field AButtonHeld? fun(): nil
---@field AButtonUp? fun(): nil
---@field BButtonDown? fun(): nil
---@field BButtonHeld? fun(): nil
---@field BButtonUp? fun(): nil
---@field downButtonDown? fun(): nil
---@field downButtonUp? fun(): nil
---@field leftButtonDown? fun(): nil
---@field leftButtonUp? fun(): nil
---@field rightButtonDown? fun(): nil
---@field rightButtonUp? fun(): nil
---@field upButtonDown? fun(): nil
---@field upButtonUp? fun(): nil
---@field cranked? fun(change:number, acceleratedChange:number): nil
```
