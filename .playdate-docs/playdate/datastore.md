# playdate.datastore

## Functions

### playdate.datastore.delete

```lua
playdate.datastore.delete(filename: string): boolean
```

Deletes the specified datastore file. The default file name is "data". Returns `false` if the datastore file could not be deleted.

### playdate.datastore.read

```lua
playdate.datastore.read(filename: string): table?
```

Returns a table instantiated with the data in the JSON-encoded file you specify. (The `.json` extension should be omitted.)  The default file name is "data". If no file is found, this function returns nil.

### playdate.datastore.readImage

```lua
playdate.datastore.readImage(path: string): _Image?
```

Reads a playdate.graphics.image from a file in the data folder. If *path* doesn’t contain a folder name, the image is searched for in a folder named "images".
`readImage()` can only load compiled pdi files. (`writeImage()` by default creates compiled pdi files.)

### playdate.datastore.write

```lua
playdate.datastore.write(table: table, filename: string, pretty: boolean): nil
```

Encodes the given table into the named file. (The `.json` extension should be omitted from the file name.) The default file name is "data". If *pretty-print* is true, the JSON will be nicely formatted.

### playdate.datastore.writeImage

```lua
playdate.datastore.writeImage(image: _Image, path: string): nil
```

Saves a playdate.graphics.image to a file. If *path* doesn’t contain a folder name, the image is stored in a folder named "images".
By default, this method writes out a PDI file, a custom image format used by Playdate that can be read back in using readImage(). If you want to write out a GIF file, append a `.gif` extension to your *path*.
Because `writeImage()` doesn’t currently support GIF transparency, if you attempt to write a GIF from an image buffer you instantiated, you must call playdate.graphics.image.new( *width, height, bgcolor* ) with *bgcolor* set to `playdate.graphics.kColorWhite` or `playdate.graphics.kColorBlack`, otherwise your image will render improperly to the file.
