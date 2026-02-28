# playdate.graphics.imagetable

## Functions

### playdate.graphics.imagetable.new

```lua
playdate.graphics.imagetable.new(count: integer, cellsWide: integer, cellSize: integer): _ImageTable
playdate.graphics.imagetable.new(path: string): (_ImageTable, string)
```

Returns an empty image table for loading images into via imagetable:load() or setting already-loaded images into with imagetable:setImage(). If set, *cellsWide* is used to locate images by x,y position. The optional *cellSize* argument gives the allocation size for the images, if load() will be used. (This is a weird technical detail, so ask us if you need guidance here.)

### playdate.graphics.imagetable:__index

```lua
playdate.graphics.imagetable:__index(n: integer): _Image?
```

Equivalent to imagetable:getImage(n).

### playdate.graphics.imagetable:__len

```lua
playdate.graphics.imagetable:__len(): integer
```

Equivalent to imagetable:getLength()
In Lua, you can get the length of a string or table using the length operator. For a `playdate.graphics.imagetable` called `myImageTable`, both `#myImageTable` and `myImageTable:getLength()` would return the same result.

### playdate.graphics.imagetable:drawImage

```lua
playdate.graphics.imagetable:drawImage(n: integer, x: integer, y: integer, flip: (integer|string)): nil
```

Equivalent to `graphics.imagetable:getImage(n):draw(x,y,[flip])`.

### playdate.graphics.imagetable:getImage

```lua
playdate.graphics.imagetable:getImage(n: integer): _Image?
playdate.graphics.imagetable:getImage(x: integer, y: integer): _Image?
```

Returns the *n*-th playdate.graphics.image in the table (ordering left-to-right, top-to-bottom). The first image is at index 1. If .n_ or (*x*,*y*) is out of bounds, the function returns nil. See also imagetable[n].

### playdate.graphics.imagetable:getLength

```lua
playdate.graphics.imagetable:getLength(): integer
```

Returns the number of images in the table. See also #imagetable.

### playdate.graphics.imagetable:getSize

```lua
playdate.graphics.imagetable:getSize(): (integer, integer)
```

Returns the pair (*cellsWide*, *cellsHigh*).

### playdate.graphics.imagetable:load

```lua
playdate.graphics.imagetable:load(path: string): (boolean, string?)
```

Loads a new image table from the data at *path* into an already-existing image table, without allocating additional memory. The image table at *path* must contain images of the same dimensions as the previous.
Returns `(success, [error])`. If the boolean `success` is false, `error` is also returned.

### playdate.graphics.imagetable:setImage

```lua
playdate.graphics.imagetable:setImage(n: integer, image: _Image): nil
```

Sets the image at slot *n* in the image table by creating a reference to the data in *image*.
