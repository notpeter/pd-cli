# playdate.network

## Functions

### playdate.network.getStatus

```lua
playdate.network.getStatus(): integer
```

### playdate.network.http.new

```lua
playdate.network.http.new(server: string, port: integer, usessl: boolean, reason: string): _NetworkHttp?
```

Returns a `playdate.network.http` object for connecting to the given server. The default port is 443 if `usessl` is true, otherwise 80; the default value for `usessl` is false. If the user has not yet given permission for the device to connect to the server, the game is paused while the system asks the user to allow or deny network access for the provided `reason`, if one is given. Since the system uses a coroutine `yield()` to show the dialog to request access (if not already given), it cannot be called at load time or from an input handler or other system callback.

### playdate.network.http.requestAccess

```lua
playdate.network.http.requestAccess(server: string, port: integer, usessl: boolean, reason: string): boolean
```

`playdate.network.http.new()` will automatically request access if needed (and note that `new()` only creates an object for connecting, doesn’t open the connection until `get()` or `post()` is called) but if you want to present the access dialog ahead of time you can use this function. Notably, this lets you request access to all HTTP servers by leaving the `server` field empty, or all subdomains of a domain by passing in the parent. Note that this function uses a coroutine `yield()` to pause the runtime while the permission dialog is up, so it can’t be called immediately at startup, must be called from a `playdate.update()` context

### playdate.network.http:close

```lua
playdate.network.http:close(): nil
```

Closes the HTTP connection. The connection may be used again for another request.

### playdate.network.http:get

```lua
playdate.network.http:get(path: string, headers: table<string, string>): (boolean, string)
```

Opens the connection to the server if it’s not already open (e.g. from a previous request with the given path and additional *headers* if specified. The *headers* argument can either be a string containing all of the headers to send (with newlines between individual headers), an array of strings, or a table of key/value pairs.
If the request is successfully queued, the function returns `true`. On error, the function returns `false` and a string indicating the error.

### playdate.network.http:getBytesAvailable

```lua
playdate.network.http:getBytesAvailable(): integer
```

Returns the number of bytes currently available for reading from the connection.

### playdate.network.http:getError

```lua
playdate.network.http:getError(): string?
```

Returns a text description of the last error on the connection, or nil if no error occurred.

### playdate.network.http:getProgress

```lua
playdate.network.http:getProgress(): (integer, integer)
```

Returns two values: the number of bytes already read from the connection and the total bytes the server plans to send.

### playdate.network.http:getResponseHeaders

```lua
playdate.network.http:getResponseHeaders(): table<string, string>?
```

Returns a table containing the key/value pairs in the HTTP response headers, or nil if no headers were received.

### playdate.network.http:getResponseStatus

```lua
playdate.network.http:getResponseStatus(): integer
```

Returns the HTTP status response code, if the request response headers have been received and parsed.

### playdate.network.http:post

```lua
playdate.network.http:post(path: string, headers: table<string, string>, data: string): (boolean, string)
playdate.network.http:post(path: string, data: string): (boolean, string)
```

Equivalent to calling `playdate.network.http:query()` with *method* equal to `POST`.

### playdate.network.http:query

```lua
playdate.network.http:query(method: string, path: string, headers: table<string, string>, data: string): (boolean, string)
playdate.network.http:query(method: string, path: string, data: string): (boolean, string)
```

Opens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends the given request with the given path, additional *headers* if specified, and the provided *data*. The *headers* argument can either be a string containing all of the headers to send (with newlines between individual headers), an array of strings, or a table of key/value pairs. If there is only one argument after *path* it is assumed to be *data*.
If the request is successfully queued, the function returns `true`. On error, the function returns `false` and a string indicating the error.

### playdate.network.http:read

```lua
playdate.network.http:read(length: integer): string
```

On success, returns up to `length` bytes (maximum 64KB) from the connection. If `length` is more than the number of bytes available the function will wait for more data up to the length of time set by `setReadTimeout()` (default one second).

### playdate.network.http:setByteRange

```lua
playdate.network.http:setByteRange(from: integer, to: integer): nil
```

Adds a `Range: bytes` header to the HTTP request.

### playdate.network.http:setConnectionClosedCallback

```lua
playdate.network.http:setConnectionClosedCallback(_function: function): nil
```

Sets a function to be called when the server has closed the connection.

### playdate.network.http:setHeadersReadCallback

```lua
playdate.network.http:setHeadersReadCallback(_function: function): nil
```

Sets a function to be called after the connection has parsed the headers from the server response. At this point, `getResponseStatus()` and `getProgress()` can be used to query the status and size of the response, and `get()`/`post()` can queue another request if `connection:setKeepAlive(true)` was set.

### playdate.network.http:setKeepAlive

```lua
playdate.network.http:setKeepAlive(flag: boolean): nil
```

If `flag` is true, this causes the HTTP request to include a *Connection: keep-alive* header.

### playdate.network.http:setReadBufferSize

```lua
playdate.network.http:setReadBufferSize(bytes: integer): nil
```

Sets the size of the connection’s read buffer.

### playdate.network.http:setRequestCallback

```lua
playdate.network.http:setRequestCallback(_function: function): nil
```

Sets a function to be called when response data is available.

### playdate.network.http:setRequestCompleteCallback

```lua
playdate.network.http:setRequestCompleteCallback(_function: function): nil
```

Sets a function to be called when all data for the request has been received (if the response contained a Content-Length header and the size is known) or the request times out.

### playdate.network.setEnabled

```lua
playdate.network.setEnabled(flag: boolean, callback: fun(error?: string)): nil
```

Playdate will connect to the configured access point automatically as needed and turn off the wifi radio after a 30 second idle timeout. This function allows a game to start connecting to the access point sooner, since that can take upwards of 10 seconds, or turn off wifi as soon as it’s no longer needed instead of waiting 30 seconds. If `flag` is true, a callback function can be provided to check for an error connecting to the access point; the argument passed to the callback is a string describing the error, or nil if no error occurred.

### playdate.network.tcp.new

```lua
playdate.network.tcp.new(server: string, port: integer, usessl: boolean, reason: string): _NetworkTcp?
```

Returns a `playdate.network.tcp` object for connecting to the given server. The default value for `usessl` is false. If the user has not yet given permission for the device to connect to the server, the game is paused while the system asks the user to allow or deny network access for the provided `reason`, if one is given. Since the system uses a coroutine `yield()` to show the dialog to request access (if not already given), it cannot be called at load time or from an input handler or other system callback.

### playdate.network.tcp.requestAccess

```lua
playdate.network.tcp.requestAccess(server: string, port: integer, reason: string): boolean
```

`playdate.network.tcp.new()` will automatically request access if needed (and note that `new()` only creates an object for connecting, doesn’t open the connection until `open()` is called) but if you want to present the access dialog ahead of time you can use this function. Notably, this lets you request access to all servers by leaving the `server` field empty, or all subdomains of a domain by passing in the parent. Access to all ports on a given server can be requested by leaving `port` empty. Note that this function uses a coroutine `yield()` to pause the runtime while the permission dialog is up, so it can’t be called immediately at startup, must be called from a `playdate.update()` context

### playdate.network.tcp:close

```lua
playdate.network.tcp:close(): nil
```

Closes the connection. `open()` may be called again after this to reopen the connection to the server.

### playdate.network.tcp:getBytesAvailable

```lua
playdate.network.tcp:getBytesAvailable(): integer
```

Returns the number of bytes currently available in the connection’s read buffer for reading from the connection.

### playdate.network.tcp:getError

```lua
playdate.network.tcp:getError(): string?
```

Returns a text description of the last error on the connection, or nil if no error occurred.

### playdate.network.tcp:open

```lua
playdate.network.tcp:open(connectCallback: fun(connected: boolean, error?: string)): nil
```

Attempts to open the TCP connection. `connectCallback` is a function to be called when the connection either succeeds or fails. The function is called with a boolean indicating whether the connection was successful, and an error string if the connection failed.
```
connection:open(function tcpConnectCallback(connected, err)
        if connected then print("connected!") else print("connection failed: "..err) end
end)
```

### playdate.network.tcp:read

```lua
playdate.network.tcp:read(length: integer): string
```

On success, returns up to `length` bytes (maximum 64KB) from the connection as well as the number of bytes that were read. If `length` is more than the number of bytes available the function will wait for more data up to the length of time set by `setReadTimeout()` (default one second).

### playdate.network.tcp:setConnectionClosedCallback

```lua
playdate.network.tcp:setConnectionClosedCallback(_function: function): nil
```

Sets a function to be called when the server has closed the connection.

### playdate.network.tcp:setReadBufferSize

```lua
playdate.network.tcp:setReadBufferSize(bytes: integer): nil
```

Sets the size of the connection’s read buffer.

### playdate.network.tcp:write

```lua
playdate.network.tcp:write(data: string): (boolean, string)
```

Attempts to write the given data to the connection. On success, returns `true`; on failure, returns `false` and a string describing the error.

## Classes

### playdate.network

```lua
---@class playdate.network
---@field kStatusNotConnected integer 0
---@field kStatusConnected integer 1
---@field kStatusNotAvailable integer 2
```
