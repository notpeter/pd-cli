# playdate.file

## Functions

### playdate.file.delete

```lua
playdate.file.delete(path: string, recursive: boolean): boolean
```

Deletes the file at the given path. Returns true if successful, else false.
If *recursive* is `true`, this function will delete the directory at *path* and its contents, otherwise the directory must be empty to be deleted.

### playdate.file.exists

```lua
playdate.file.exists(path: string): boolean
```

Returns true if a file exists at the given path. Unlike the image or sound loading functions, this function requires *path* to include the file extension since it cannot be inferred from context. Additionally, note that asset files are compiled into a format easier for Playdate to use and will have a different extension: `.wav` and `.aiff` audio files are compiled to `.pda` format, and `.gif` and `.png` files become `.pdi`s.

### playdate.file.file:close

```lua
playdate.file.file:close(): nil
```

Closes the file.
Equivalent to `playdate->file->close()` in the C API.

### playdate.file.file:flush

```lua
playdate.file.file:flush(): nil
```

Flushes any buffered data written to the file to the disk.
Equivalent to `playdate->file->flush()` in the C API.

### playdate.file.file:read

```lua
playdate.file.file:read(numberOfBytes: integer): (integer, string?)
```

Returns a buffer containing up to *numberOfBytes* bytes from the file, and the number of bytes read. If the read failed, the function returns `nil` and a second value describing the error.
Equivalent to `playdate->file->read()` in the C API.

### playdate.file.file:readline

```lua
playdate.file.file:readline(): string
```

Returns the next line of the file, delimited by either `\n` or `\r\n`. The returned string does not include newline characters.

### playdate.file.file:seek

```lua
playdate.file.file:seek(offset: integer, whence: integer): nil
```

Sets the file read/write position to the given byte offset. `whence`, if given is one of the following:
* **playdate.file.kSeekSet**: `offset` is an absolute offset from the start of the file
* **playdate.file.kSeekFromCurrent**: `offset` is relative to the current position
* **playdate.file.kSeekFromEnd**: `offset` is an offset from the end of the file (negative values are before the end, positive are past the end)
Equivalent to `playdate->file->seek()` in the C API.

### playdate.file.file:tell

```lua
playdate.file.file:tell(): integer
```

Returns the current byte offset of the read/write position in the file.
Equivalent to `playdate->file->tell()` in the C API.

### playdate.file.file:write

```lua
playdate.file.file:write(str: string): (integer, string?)
```

Writes the given string to the file and returns the number of bytes written if successful, or 0 and a second return value describing the error. If you wish to include line termination characters (`\n`, `\r`), please include them in the string.

### playdate.file.getSize

```lua
playdate.file.getSize(path: string): integer
```

Returns the size of the file at the given path.

### playdate.file.getType

```lua
playdate.file.getType(path: string): string
```

Returns the type of the file at the given path.

### playdate.file.isdir

```lua
playdate.file.isdir(path: string): boolean
```

Returns true if a directory exists at the given path.

### playdate.file.listFiles

```lua
playdate.file.listFiles(path: string, showhidden: boolean): string[]
```

Returns an array containing the file names in the given directory path as strings. Folders are indicated by a slash `/` at the end of the filename. If *showhidden* is set, files beginning with a period will be included; otherwise, they are skipped.
Call with no argument to get a list of all files and folders your game has access to. (For a game with default access permissions, `listFiles()`, `listFiles("/")`, and `listFiles(".")` should all return the same result.)
Equivalent to `playdate->file->listfiles()` in the C API.
Learn more about the Playdate filesystem.

### playdate.file.load

```lua
playdate.file.load(path: string, env: table): function
```

Loads the compiled *.pdz* file at the given location and returns the contents as a function. The .pdz extension on *path* is optional.
*env*, if specified, is a table to use as the function’s global namespace instead of *_G*.

### playdate.file.mkdir

```lua
playdate.file.mkdir(path: string): nil
```

Creates a directory at the given path, under the /Data/{bundleid} folder. See About the Playdate Filesystem for details.
`playdate.file.mkdir()` will create all intermediate directories, if a succession of directories ("testdir/testdir/testdir/") is specified in *path*.
Equivalent to `playdate->file->mkdir()` in the C API.

### playdate.file.open

```lua
playdate.file.open(path: string, mode: integer): (_File?, string?)
```

Returns a playdate.file.file corresponding to the opened file. *mode* should be one of the following:
* **playdate.file.kFileRead**: the file is opened for reading; the system first looks in the /Data/{bundleid} folder for the given file, then in the game’s pdx folder if it isn’t found
* **playdate.file.kFileWrite**: the file is created if it doesn’t exist, truncated to zero length if it does, then opened for writing
* **playdate.file.kFileAppend**: the file is created if it doesn’t exist, opened for writing, with new data written to the end of the file
If *mode* is not specified, the default is *playdate.file.kFileRead*.
If the file couldn’t be opened, a second return value indicates the error. The filesystem has a limit of 64 simultaneous open files.
Equivalent to `playdate->file->open()` in the C API.

### playdate.file.rename

```lua
playdate.file.rename(path: string, newPath: string): boolean
```

Renames the file at *path*, if it exists, to the value of newPath. This can result in the file being moved to a new directory, but directories will not be created. Returns true if the operation was successful.
Equivalent to `playdate->file->rename()` in the C API.

### playdate.file.run

```lua
playdate.file.run(path: string, env: table): nil
```

Runs the pdz file at the given location. Equivalent to `playdate.file.load(path, env)()`.
The *.pdz* extension on *path* is optional. Values returned from the pdz file are left on the stack.
*env*, if specified, is a table to use as the function’s global namespace instead of *_G*.

## Classes

### playdate.file

```lua
---@class playdate.file
---@field kSeekSet integer 0
---@field kSeekFromCurrent integer 1
---@field kSeekFromEnd integer 2
---@field kFileRead integer 3
---@field kFileWrite integer 4
---@field kFileAppend integer 8
```
