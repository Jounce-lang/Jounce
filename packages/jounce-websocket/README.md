# jounce-websocket

WebSocket client/server with presence tracking, rooms, automatic reconnection, and event handling.

## Features

- ✅ **WebSocket Client** - Full-featured WebSocket client
- ✅ **WebSocket Server** - WebSocket server with connection management
- ✅ **Automatic Reconnection** - Configurable reconnect logic
- ✅ **Message Queue** - Queue messages when offline
- ✅ **Rooms/Channels** - Join/leave rooms for group messaging
- ✅ **Presence Tracking** - Online/offline/away status
- ✅ **Event Handlers** - React to connect/disconnect/message events
- ✅ **Connection Management** - Track active connections

## Installation

```bash
jnc pkg add jounce-websocket
```

## Quick Start

### Client

```jounce
use jounce_websocket::{WebSocketClient, Message};

// Create and connect
let mut client = WebSocketClient::new("ws://localhost:8080")
    .with_reconnect(true)
    .with_max_reconnect_attempts(5)
    .connect();

// Send message
client = client.send_text("chat", "Hello!");

// Join room
client = client.join_room("lobby");
```

### Server

```jounce
use jounce_websocket::{WebSocketServer, Connection};

// Create and start server
let mut server = WebSocketServer::new(8080)
    .start();

// Add connection
let conn = Connection::new("conn1");
server = server.add_connection("conn1", conn);

// Create room
server = server.create_room("lobby");

// Join room
server = server.join_room("conn1", "lobby");
```

## Usage

### WebSocket Client

```jounce
use jounce_websocket::WebSocketClient;

// Create client with reconnection
let mut client = WebSocketClient::new("ws://example.com")
    .with_reconnect(true)
    .with_max_reconnect_attempts(10)
    .with_reconnect_delay(2000);

// Connect
client = client.connect();

// Check connection
if client.is_connected() {
    println("Connected!");
}

// Send messages
let message = Message::new("event", "data");
client = client.send(message);

// Or use convenience method
client = client.send_text("chat", "Hello world!");
```

### Message Queuing

```jounce
// Messages sent while disconnected are queued
let mut client = WebSocketClient::new("ws://example.com");

// Send before connecting - goes to queue
client = client.send_text("early", "message");

// Connect and flush queue automatically
client = client.connect();
// "early" message is now sent
```

### Rooms/Channels

```jounce
// Join multiple rooms
client = client
    .join_room("general")
    .join_room("announcements");

// Check room membership
if client.is_in_room("general") {
    println("In general room");
}

// Leave room
client = client.leave_room("general");
```

### WebSocket Server

```jounce
use jounce_websocket::{WebSocketServer, Connection, Message};

// Create server
let mut server = WebSocketServer::new(8080);

// Start server
server = server.start();

// Add connection
let conn = Connection::new("user123");
server = server.add_connection("user123", conn);

// Get connection count
let count = server.get_connection_count();
println("Connections: " + count.to_string());

// Broadcast to all
let msg = Message::new("announcement", "Server starting");
server.broadcast(msg);

// Send to specific connection
server.send_to("user123", msg);
```

### Rooms on Server

```jounce
// Create room
server = server.create_room("lobby");

// Join room
server = server.join_room("user123", "lobby");

// Broadcast to room
let msg = Message::new("chat", "Hello lobby!");
server.broadcast_to_room("lobby", msg);

// Get room members
let members = server.get_room_members("lobby");
println("Members: " + members.len().to_string());

// Leave room
server = server.leave_room("user123", "lobby");
```

### Presence Tracking

```jounce
use jounce_websocket::{PresenceTracker, PresenceStatus};

let mut tracker = PresenceTracker::new();

// Set user online
tracker = tracker.set_online("user1");

// Check status
if tracker.is_online("user1") {
    println("User is online");
}

// Set away
tracker = tracker.set_away("user1");

// Set offline
tracker = tracker.set_offline("user1");

// Get online users
let online = tracker.get_online_users();
println("Online: " + tracker.get_online_count().to_string());
```

### Event Handlers

```jounce
use jounce_websocket::EventHandler;

let handler = EventHandler::new()
    .on_connect(|id| {
        println("Connected: " + id);
    })
    .on_disconnect(|id| {
        println("Disconnected: " + id);
    })
    .on_message(|msg| {
        println("Message: " + msg.data);
    })
    .on_error(|err| {
        println("Error: " + err);
    });

// Trigger events
handler.trigger_connect("user123");
handler.trigger_message(msg);
```

### Connection Management

```jounce
use jounce_websocket::Connection;

// Create connection
let mut conn = Connection::new("user123");

// Set user data
conn = conn
    .set_user_data("username", "john")
    .set_user_data("role", "admin");

// Get user data
let username = conn.get_user_data("username");

// Update activity
conn = conn.update_activity();

// Check if active
if conn.is_active() {
    println("Connection is active");
}
```

### Room with Metadata

```jounce
use jounce_websocket::Room;

let mut room = Room::new("game-room");

// Add members
room = room
    .add_member("player1")
    .add_member("player2");

// Set metadata
room = room
    .set_metadata("max_players", "4")
    .set_metadata("game_mode", "deathmatch");

// Get metadata
let max_players = room.get_metadata("max_players");

// Check member
if room.has_member("player1") {
    println("Player 1 is in room");
}

// Get count
let count = room.get_member_count();
```

## API Reference

### WebSocketClient

```jounce
WebSocketClient::new(url: string) -> WebSocketClient
client.with_reconnect(enabled: bool) -> WebSocketClient
client.with_max_reconnect_attempts(max: int) -> WebSocketClient
client.with_reconnect_delay(delay: int) -> WebSocketClient
client.connect() -> WebSocketClient
client.disconnect() -> WebSocketClient
client.send(message: Message) -> WebSocketClient
client.send_text(type: string, data: string) -> WebSocketClient
client.join_room(room: string) -> WebSocketClient
client.leave_room(room: string) -> WebSocketClient
client.is_in_room(room: string) -> bool
client.is_connected() -> bool
client.get_state() -> ConnectionState
```

### WebSocketServer

```jounce
WebSocketServer::new(port: int) -> WebSocketServer
server.start() -> WebSocketServer
server.stop() -> WebSocketServer
server.add_connection(id: string, conn: Connection) -> WebSocketServer
server.remove_connection(id: string) -> WebSocketServer
server.get_connection_count() -> int
server.create_room(name: string) -> WebSocketServer
server.join_room(conn_id: string, room: string) -> WebSocketServer
server.leave_room(conn_id: string, room: string) -> WebSocketServer
server.broadcast(message: Message)
server.broadcast_to_room(room: string, message: Message)
server.send_to(conn_id: string, message: Message)
server.get_room_members(room: string) -> Array<string>
```

### PresenceTracker

```jounce
PresenceTracker::new() -> PresenceTracker
tracker.set_online(user_id: string) -> PresenceTracker
tracker.set_offline(user_id: string) -> PresenceTracker
tracker.set_away(user_id: string) -> PresenceTracker
tracker.get_status(user_id: string) -> PresenceStatus
tracker.is_online(user_id: string) -> bool
tracker.get_online_users() -> Array<string>
tracker.get_online_count() -> int
```

### Message

```jounce
Message::new(type: string, data: string) -> Message
message.to_json() -> string
Message::from_json(json: string) -> Message
```

### Connection

```jounce
Connection::new(id: string) -> Connection
conn.update_activity() -> Connection
conn.set_user_data(key: string, value: string) -> Connection
conn.get_user_data(key: string) -> Option<string>
conn.is_active() -> bool
```

### Room

```jounce
Room::new(name: string) -> Room
room.add_member(conn_id: string) -> Room
room.remove_member(conn_id: string) -> Room
room.has_member(conn_id: string) -> bool
room.get_member_count() -> int
room.set_metadata(key: string, value: string) -> Room
room.get_metadata(key: string) -> Option<string>
```

## Best Practices

1. **Enable Reconnection** - Always enable automatic reconnection for clients
2. **Handle Events** - Use EventHandler to react to connection changes
3. **Track Presence** - Use PresenceTracker for user online status
4. **Use Rooms** - Group related connections with rooms
5. **Queue Messages** - Client automatically queues when disconnected
6. **Cleanup** - Remove connections when they disconnect
7. **Metadata** - Use connection and room metadata for custom data

## Examples

See `tests/` directory for comprehensive usage examples.

## License

MIT

## Version

0.1.0
