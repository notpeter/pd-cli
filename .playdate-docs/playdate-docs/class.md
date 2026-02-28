# class

## Functions

### Object.baseObject

```lua
Object.baseObject(): table
```

### Object:init

```lua
Object:init(...: any): nil
```

### Object:isa

```lua
Object:isa(Class: table): boolean
```

### Object:tableDump

```lua
Object:tableDump(indent: boolean, _table: table): nil
```

### class

```lua
class(ClassName: string, properties: table, namespace: table): _NewClass
```

## Classes

### Object

```lua
---@class Object
---@field class table
---@field className string
```

### _NewClass

```lua
---@class _NewClass
---@field className string
---@field properties? table
---@field namespace? table
---@field extends fun(Parent?: (table|string)): nil
```

### _Object

```lua
---@class _Object : Object
---@field super table
```
