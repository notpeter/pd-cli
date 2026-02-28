# table

## Functions

### table.create

```lua
table.create(arrayCount: integer, hashCount: integer): table
```

Returns a new Lua table with the array and hash parts preallocated to accommodate *arrayCount* and *hashCount* elements respectively.
If you can make a decent estimation of how big your table will need to be, `table.create()` can be much more efficient than the alternative, especially in loops. For example, if you know your array is always going to contain approximately ten elements, say `myArray = table.create( 10, 0 )` instead of `myArray = {}`.

### table.deepcopy

```lua
table.deepcopy(source: table): table
```

`deepcopy` returns a deep copy of the *source* table. The copy will contain copies of any nested tables.

### table.getsize

```lua
table.getsize(table: table): (integer, integer)
```

Returns the size of the given table as multiple values (*arrayCount*, *hashCount*).

### table.indexOfElement

```lua
table.indexOfElement(table: table, element: any): integer?
```

Returns the first index of *element* in the given array-style table. If the table does not contain *element*, the function returns nil.

### table.shallowcopy

```lua
table.shallowcopy(source: table, destination: table): table
```

`shallowcopy` returns a shallow copy of the *source* table. If a *destination* table is provided, it copies the contents of *source* into *destination* and returns *destination*. The copy will contain references to any nested tables.
