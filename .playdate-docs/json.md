# json

## Functions

### json.decode

```lua
json.decode(str: string): table
```

Takes the JSON encoded string and converts it to a Lua table.
Equivalent to `playdate->json->decode()` in the C API.

### json.decodeFile

```lua
json.decodeFile(file: _File): table
json.decodeFile(path: string): table
```

Reads the given playdate.file.file object or the file at the given `path` and converts it to a Lua table.

### json.encode

```lua
json.encode(table: table): string
```

Returns a string containing the JSON representation of the passed-in Lua table.

### json.encodePretty

```lua
json.encodePretty(table: table): string
```

Returns a string containing the JSON representation of a Lua table, with human-readable formatting.

### json.encodeToFile

```lua
json.encodeToFile(file: _File, pretty: boolean, table: table): nil
json.encodeToFile(path: string, pretty: boolean, table: table): nil
```

Encodes the Lua table `table` to JSON and writes it to the given playdate.file.file object or the given `path`. If `pretty` is true, the output is formatted to make it human-readable. Otherwise, no additional whitespace is added.
For a very simple way to serialize a table to a file, see playdate.datastore.
