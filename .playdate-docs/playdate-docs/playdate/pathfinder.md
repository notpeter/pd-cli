# playdate.pathfinder

## Functions

### playdate.pathfinder.graph.new

```lua
playdate.pathfinder.graph.new(nodeCount: integer, coordinates: integer[][]): _PathFinderGraph
```

Returns a new empty playdate.pathfinder.graph object.
If `nodeCount` is supplied, that number of nodes will be allocated and added to the graph. Their IDs will be set from 1 to `nodeCount`.
`coordinates`, if supplied, should be a table containing tables of x, y values, indexed by node IDs. For example, `{{10, 10}, {50, 30}, {20, 100}, {100, 120}, {160, 130}}`.

### playdate.pathfinder.graph.new2DGrid

```lua
playdate.pathfinder.graph.new2DGrid(width: integer, height: integer, allowDiagonals: boolean, includedNodes: integer[]): _PathFinderGraph
```

Convenience function that returns a new playdate.pathfinder.graph object containing nodes for for each grid position, even if not connected to any other nodes. This allows for easier graph modification once the graph is generated. Weights for connections between nodes are set to 10 for horizontal and vertical connections and 14 for diagonal connections (if included), as this tends to produce nicer paths than using uniform weights. Nodes have their indexes set from 1 to *width* * *height*, and have their *x, y* values set appropriately for the node’s position.
* *width*: The width of the grid to be created.
* *height*: The height of the grid to be created.
* *allowDiagonals*: If true, diagonal connections will also be created.
* *includedNodes*: A one-dimensional array of length *width* * *height*. Each entry should be a 1 or a 0 to indicate nodes that should be connected to their neighbors and nodes that should not have any connections added. If not provided, all nodes will be connected to their neighbors.

### playdate.pathfinder.graph:addConnectionToNodeWithID

```lua
playdate.pathfinder.graph:addConnectionToNodeWithID(fromNodeID: integer, toNodeID: integer, weight: number, addReciprocalConnection: boolean): nil
```

Adds a connection from the node with `id` `fromNodeID` to the node with `id` `toNodeID` with a weight value of `weight`. Weights affect the path the A* algorithm will solve for. A longer, lighter-weighted path will be chosen over a shorter heavier path, if available. If `addReciprocalConnection` is true, the reverse connection will also be added.

### playdate.pathfinder.graph:addConnections

```lua
playdate.pathfinder.graph:addConnections(connections: integer[][]): nil
```

`connections` should be a table of array-style tables. The keys of the outer table should correspond to node IDs, while the inner array should be a series if connecting node ID and weight combinations that will be assigned to that node. For example, `{[1]={2, 10, 3, 12}, [2]={1, 20}, [3]={1, 20, 2, 10}}` will create a connection from node ID 1 to node ID 2 with a weight of 10, and a connection to node ID 3 with a weight of 12, and so on for the other entries.

### playdate.pathfinder.graph:addNewNode

```lua
playdate.pathfinder.graph:addNewNode(id: integer, x: integer, y: integer, connectedNodes: _PathFinderNode[], weights: number[], addReciprocalConnections: boolean): nil
```

Creates a new playdate.pathfinder.node and adds it to the graph.
* *id*: id value for the new node.
* *x*: Optional x value for the node.
* *y*: Optional y value for the node.
* *connectedNodes*: Array of existing nodes to create connections to from the new node.
* *weights*: Array of weights for the new connections. Array must be the same length as *connectedNodes*. Weights affect the path the A* algorithm will solve for. A longer, lighter-weighted path will be chosen over a shorter heavier path, if available.
* *addReciprocalConnections*: If true, connections will also be added in the reverse direction for each node.

### playdate.pathfinder.graph:addNewNodes

```lua
playdate.pathfinder.graph:addNewNodes(count: integer): nil
```

Creates *count* new nodes, adding them to the graph, and returns them in an array-style table. The new node’s *id_s will be assigned values 1 through _count*-1.
This method is useful to improve performance if many nodes need to be allocated at once rather than one at a time, for example when creating a new graph.

### playdate.pathfinder.graph:addNode

```lua
playdate.pathfinder.graph:addNode(node: _PathFinderNode, connectedNodes: _PathFinderNode[], weights: number[], addReciprocalConnections: boolean): nil
```

Adds an already-existing node to the graph. The node must have originally belonged to the same graph.
* *node*: Node to be added to the graph.
* *connectedNodes*: Array of existing nodes to create connections to from the new node.
* *weights*: Array of weights for the new connections. Array must be the same length as *connectedNodes*. Weights affect the path the A* algorithm will solve for. A longer, lighter-weighted path will be chosen over a shorter heavier path, if available.
* *addReciprocalConnections*: If true, connections will also be added in the reverse direction for each connection added.

### playdate.pathfinder.graph:addNodes

```lua
playdate.pathfinder.graph:addNodes(nodes: _PathFinderNode[]): nil
```

Adds an array of already-existing nodes to the graph.

### playdate.pathfinder.graph:allNodes

```lua
playdate.pathfinder.graph:allNodes(): _PathFinderNode[]
```

Returns an array containing all nodes in the graph.

### playdate.pathfinder.graph:findPath

```lua
playdate.pathfinder.graph:findPath(startNode: _PathFinderNode, goalNode: _PathFinderNode, heuristicFunction: fun(startNode: _PathFinderNode, goalNode: _PathFinderNode): integer, findPathToGoalAdjacentNodes: boolean): _PathFinderNode[]
```

Returns an array of nodes representing the path from *startNode* to *goalNode*, or *nil* if no path can be found.
* *heuristicFunction*: If provided, this function should be of the form *function(startNode, goalNode)* and should return an integer value estimate or underestimate of the distance from *startNode* to *goalNode*. If not provided, a manhattan distance function will be used to calculate the estimate. This requires that the *x, y* values of the nodes in the graph have been set properly.
* *findPathToGoalAdjacentNodes*: If true, a path will be found to any node adjacent to the goal node, based on the *x, y* values of those nodes and the goal node. This does not rely on connections between adjacent nodes and  the goal node, which can be entirely disconnected from the rest of the graph.

### playdate.pathfinder.graph:findPathWithIDs

```lua
playdate.pathfinder.graph:findPathWithIDs(startNodeID: integer, goalNodeID: integer, heuristicFunction: fun(startNode: _PathFinderNode, goalNode: _PathFinderNode): integer, findPathToGoalAdjacentNodes: boolean): integer[]
```

Works the same as findPath, but looks up nodes to find a path between using startNodeID and goalNodeID and returns a list of nodeIDs rather than the nodes themselves.

### playdate.pathfinder.graph:nodeWithID

```lua
playdate.pathfinder.graph:nodeWithID(id: integer): _PathFinderNode?
```

Returns the first node found in the graph with a matching *id*, or nil if no such node is found.

### playdate.pathfinder.graph:nodeWithXY

```lua
playdate.pathfinder.graph:nodeWithXY(x: integer, y: integer): _PathFinderNode?
```

Returns the first node found in the graph with matching *x* and *y* values, or nil if no such node is found.

### playdate.pathfinder.graph:removeAllConnections

```lua
playdate.pathfinder.graph:removeAllConnections(): nil
```

Removes all connections from all nodes in the graph.

### playdate.pathfinder.graph:removeAllConnectionsFromNodeWithID

```lua
playdate.pathfinder.graph:removeAllConnectionsFromNodeWithID(id: integer, removeIncoming: boolean): nil
```

Removes all connections from the matching node.
If `removeIncoming` is true, all connections from other nodes to the calling node are also removed. False by default. Please note: this can signficantly increase the time this function takes as it requires a full search of the graph - O(1) vs O(n)).

### playdate.pathfinder.graph:removeNode

```lua
playdate.pathfinder.graph:removeNode(node: _PathFinderNode): nil
```

Removes node from the graph. Also removes all connections to and from the node.

### playdate.pathfinder.graph:removeNodeWithID

```lua
playdate.pathfinder.graph:removeNodeWithID(id: integer): nil
```

Returns the first node found with a matching *id*, after removing it from the graph and removing all connections to and from the node.

### playdate.pathfinder.graph:removeNodeWithXY

```lua
playdate.pathfinder.graph:removeNodeWithXY(x: integer, y: integer): nil
```

Returns the first node found with coordinates matching *x, y*, after removing it from the graph and removing all connections to and from the node.

### playdate.pathfinder.graph:setXYForNodeWithID

```lua
playdate.pathfinder.graph:setXYForNodeWithID(id: integer, x: integer, y: integer): nil
```

Sets the matching node’s `x` and `y` values.

### playdate.pathfinder.node:addConnection

```lua
playdate.pathfinder.node:addConnection(node: _PathFinderNode, weight: number, addReciprocalConnection: boolean): nil
```

Adds a new connection between nodes.
* *node*: The node the new connection will point to.
* *weight*: Weight for the new connection. Weights affect the path the A* algorithm will solve for. A longer, lighter-weighted path will be chosen over a shorter heavier path, if available.
* *addReciprocalConnection*: If true, a second connection will be created with the same weight in the opposite direction.

### playdate.pathfinder.node:addConnectionToNodeWithXY

```lua
playdate.pathfinder.node:addConnectionToNodeWithXY(x: integer, y: integer, weight: number, addReciprocalConnection: boolean): nil
```

Adds a connection to the first node found with matching *x* and *y* values, if it exists.
* *weight*: The weight for the new connection. Weights affect the path the A* algorithm will solve for. A longer, lighter-weighted path will be chosen over a shorter heavier path, if available.
* *addReciprocalConnections*: If true, a connection will also be added in the reverse direction, from the node at x, y to the caller.

### playdate.pathfinder.node:addConnections

```lua
playdate.pathfinder.node:addConnections(nodes: _PathFinderNode[], weights: number[], addReciprocalConnections: boolean): nil
```

Adds a new connection to each node in the nodes array.
* *nodes*: An array of nodes which the new connections will point to.
* *weights*: An array of weights for the new connections. Must be of the same length as the nodes array. Weights affect the path the A* algorithm will solve for. A longer, lighter-weighted path will be chosen over a shorter heavier path, if available.
* *addReciprocalConnections*: If true, connections will also be added in the reverse direction for each node.

### playdate.pathfinder.node:connectedNodes

```lua
playdate.pathfinder.node:connectedNodes(): _PathFinderNode[]
```

Returns an array of nodes that have been added as connections to this node.

### playdate.pathfinder.node:removeAllConnections

```lua
playdate.pathfinder.node:removeAllConnections(removeIncoming: boolean): nil
```

Removes all connections from the calling node.
If `removeIncoming` is true, all connections from other nodes to the calling node are also removed. False by default. Please note: this can signficantly increase the time this function takes as it requires a full search of the graph - O(1) vs O(n)).

### playdate.pathfinder.node:removeConnection

```lua
playdate.pathfinder.node:removeConnection(node: _PathFinderNode, removeReciprocal: boolean): nil
```

Removes a connection to node, if it exists. If *removeReciprocal* is true the reverse connection will also be removed, if it exists.

### playdate.pathfinder.node:setXY

```lua
playdate.pathfinder.node:setXY(x: integer, y: integer): nil
```

Sets the *x* and *y* values for the node.

## Classes

### _PathFinderNode

```lua
---@class _PathFinderNode : playdate.pathfinder.node
---@field x integer
---@field y integer
---@field id integer
```
