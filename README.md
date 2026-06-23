# Protocols

Small Rust examples for REST, Server-Sent Events, WebSocket, JSON-RPC 2.0, MCP, A2A, and gRPC.

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

### stdio

Build the stdio MCP server and client:

```sh
cargo build --bin mcp_stdio_server --bin mcp_stdio_client
```

Run the Rust client. It starts the server, lists its tools, and calls `add`:

```sh
cargo run --bin mcp_stdio_client
```

Open the server with [MCP Inspector](https://modelcontextprotocol.io/docs/tools/inspector):

```sh
npx @modelcontextprotocol/inspector ./target/debug/mcp_stdio_server
```

List the available tools from the command line:

```sh
npx @modelcontextprotocol/inspector --cli ./target/debug/mcp_stdio_server \
  --method tools/list
```

Call `echo`:

```sh
npx @modelcontextprotocol/inspector --cli ./target/debug/mcp_stdio_server \
  --method tools/call \
  --tool-name echo \
  --tool-arg message=hello
```

Call `add`:

```sh
npx @modelcontextprotocol/inspector --cli ./target/debug/mcp_stdio_server \
  --method tools/call \
  --tool-name add \
  --tool-arg a=2 \
  --tool-arg b=3
```

### Streamable HTTP

Start the MCP server on `http://127.0.0.1:3002/mcp`:

```sh
cargo run --bin mcp_http_server
```

Run the HTTP client in another terminal. It lists the tools and calls `add`:

```sh
cargo run --bin mcp_http_client
```

## A2A

Start the JSON-RPC A2A server on port `3003`:

```sh
cargo run --bin a2a_server
```

Read its Agent Card:

```sh
curl http://127.0.0.1:3003/.well-known/agent-card.json | jq
```

Run the A2A client in another terminal. It sends a message, reads the stored task, and receives streaming task updates:

```sh
cargo run --bin a2a_client
```

## gRPC

Cargo runs `build.rs` automatically during the build. It compiles `proto/protocols.proto` into Rust code in Cargo's `OUT_DIR`, so no separate code-generation command is needed.

Start the gRPC server on port `50051`:

```sh
cargo run --bin grpc_server
```

Run the client in another terminal. It exercises unary, server-streaming, client-streaming, and bidirectional-streaming RPCs:

```sh
cargo run --bin grpc_client
```
