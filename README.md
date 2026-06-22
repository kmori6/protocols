# Protocols

Small Rust examples for REST, Server-Sent Events, WebSocket, JSON-RPC 2.0, and MCP.

## REST, SSE, and WebSocket server

Start the Axum server on port `3000`:

```sh
cargo run
```

### REST

```sh
# POST /items — Create an item
curl -X POST http://localhost:3000/items \
  -H 'Content-Type: application/json' \
  -d '{"name":"first item"}'

# GET /items — List all items
curl http://localhost:3000/items

# GET /items/1 — Get one item
curl http://localhost:3000/items/1

# PUT /items/1 — Update an item
curl -X PUT http://localhost:3000/items/1 \
  -H 'Content-Type: application/json' \
  -d '{"name":"updated item"}'

# DELETE /items/1 — Delete an item
curl -X DELETE http://localhost:3000/items/1
```

### Server-Sent Events

Stream the one-second ticker:

```sh
curl -N http://localhost:3000/sse/ticker
```

Open a broadcast subscription in one terminal:

```sh
curl -N http://localhost:3000/sse/broadcast
```

Publish from another terminal:

```sh
curl -X POST http://localhost:3000/sse/broadcast -d 'hello SSE'
```

### WebSocket

The following commands require [`websocat`](https://github.com/vi/websocat).

Test the echo endpoint:

```sh
websocat ws://localhost:3000/ws/echo
```

Open the chat endpoint in two terminals, then send a message from either one:

```sh
websocat ws://localhost:3000/ws/chat
```

## JSON-RPC 2.0

Start the `jsonrpsee` server on port `3001`:

```sh
cargo run --bin json_rpc
```

Test `echo`:

```sh
curl -X POST http://localhost:3001 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"echo","params":{"message":"hello"},"id":1}'
```

Test `add`:

```sh
curl -X POST http://localhost:3001 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"add","params":{"a":2,"b":3},"id":2}'
```

## MCP

Build the stdio MCP server and Rust client:

```sh
cargo build --bin mcp --bin mcp_client
```

Run the Rust client. It starts the server, lists its tools, and calls `add`:

```sh
cargo run --bin mcp_client
```

Open the server with [MCP Inspector](https://modelcontextprotocol.io/docs/tools/inspector):

```sh
npx @modelcontextprotocol/inspector ./target/debug/mcp
```

List the available tools from the command line:

```sh
npx @modelcontextprotocol/inspector --cli ./target/debug/mcp \
  --method tools/list
```

Call `echo`:

```sh
npx @modelcontextprotocol/inspector --cli ./target/debug/mcp \
  --method tools/call \
  --tool-name echo \
  --tool-arg message=hello
```

Call `add`:

```sh
npx @modelcontextprotocol/inspector --cli ./target/debug/mcp \
  --method tools/call \
  --tool-name add \
  --tool-arg a=2 \
  --tool-arg b=3
```
