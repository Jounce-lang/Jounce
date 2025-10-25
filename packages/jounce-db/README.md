# jounce-db

Database abstraction layer with PostgreSQL and SQLite support for Jounce applications.

## Features

- ✅ **PostgreSQL Adapter** - Connect to PostgreSQL databases
- ✅ **SQLite Adapter** - Connect to SQLite databases
- ✅ **Connection Pooling** - Efficient connection management
- ✅ **Query Builder** - Fluent API for building SQL queries
- ✅ **Transactions** - BEGIN, COMMIT, ROLLBACK support
- ✅ **Prepared Statements** - SQL injection protection

## Installation

```bash
jnc pkg add jounce-db
```

## Quick Start

```jounce
use jounce_db::{DbConfig, create_pool, get_connection, QueryBuilder};

fn main() {
    // Configure database
    let config = DbConfig {
        driver: "postgres",
        host: "localhost",
        port: 5432,
        database: "myapp",
        username: "user",
        password: "pass",
        pool_size: 10,
        timeout: 30,
    };

    // Create connection pool
    let pool = create_pool(config);

    // Get connection
    let conn = get_connection(pool).unwrap();

    // Query using builder
    let users = QueryBuilder::new()
        .select_all()
        .from("users")
        .where_eq("active", true)
        .fetch(conn)
        .unwrap();

    println!("Found {} users", users.len());
}
```

## Usage

### Database Configuration

```jounce
use jounce_db::DbConfig;

// PostgreSQL configuration
let pg_config = DbConfig {
    driver: "postgres",
    host: "localhost",
    port: 5432,
    database: "myapp",
    username: "admin",
    password: "secret",
    pool_size: 20,
    timeout: 30,
};

// SQLite configuration
let sqlite_config = DbConfig {
    driver: "sqlite",
    host: "",
    port: 0,
    database: "./data/myapp.db",
    username: "",
    password: "",
    pool_size: 5,
    timeout: 10,
};
```

### Connection Pooling

```jounce
use jounce_db::{create_pool, get_connection, release_connection, close_pool};

// Create pool
let mut pool = create_pool(config);

// Get connection from pool
match get_connection(pool) {
    Ok(conn) => {
        // Use connection
        let result = query(conn, "SELECT * FROM users");

        // Release back to pool
        release_connection(pool, conn);
    },
    Err(error) => {
        println!("No available connections: {}", error);
    }
}

// Close all connections
close_pool(pool);
```

### Raw SQL Queries

```jounce
use jounce_db::{query, query_one, execute};

// Query multiple rows
let rows = query(conn, "SELECT * FROM users WHERE active = true").unwrap();

for row in rows {
    let id: int = row.get("id").unwrap();
    let name: string = row.get("name").unwrap();
    println!("User {}: {}", id, name);
}

// Query single row
match query_one(conn, "SELECT * FROM users WHERE id = 1") {
    Ok(Some(row)) => {
        let name: string = row.get("name").unwrap();
        println!("Found user: {}", name);
    },
    Ok(None) => println!("No user found"),
    Err(error) => println!("Query error: {}", error),
}

// Execute DML (INSERT, UPDATE, DELETE)
let result = execute(conn, "DELETE FROM users WHERE active = false").unwrap();
println!("Deleted {} rows", result.rows_affected);
```

### Prepared Statements

```jounce
use jounce_db::{query_prepared, execute_prepared};

// Query with parameters (prevents SQL injection)
let params = vec![1, true];
let rows = query_prepared(
    conn,
    "SELECT * FROM users WHERE id = ? AND active = ?",
    params
).unwrap();

// Insert with parameters
let insert_params = vec!["Alice", "alice@example.com", 25];
let result = execute_prepared(
    conn,
    "INSERT INTO users (name, email, age) VALUES (?, ?, ?)",
    insert_params
).unwrap();

println!("Inserted row with ID: {}", result.last_insert_id);
```

### Query Builder

```jounce
use jounce_db::QueryBuilder;

// SELECT query
let users = QueryBuilder::new()
    .select(vec!["id", "name", "email"])
    .from("users")
    .where_eq("active", true)
    .order_by("name", "ASC")
    .limit(10)
    .offset(0)
    .fetch(conn)
    .unwrap();

// SELECT all columns
let all_users = QueryBuilder::new()
    .select_all()
    .from("users")
    .fetch(conn)
    .unwrap();

// SELECT with complex WHERE
let filtered = QueryBuilder::new()
    .select_all()
    .from("users")
    .where("age > ? AND age < ?", vec![18, 65])
    .where_eq("country", "USA")
    .fetch(conn)
    .unwrap();

// SELECT single row
match QueryBuilder::new()
    .select_all()
    .from("users")
    .where_eq("email", "alice@example.com")
    .fetch_one(conn)
{
    Ok(Some(row)) => {
        let name: string = row.get("name").unwrap();
        println!("Found: {}", name);
    },
    Ok(None) => println!("Not found"),
    Err(error) => println!("Error: {}", error),
}
```

### INSERT Queries

```jounce
use jounce_db::QueryBuilder;

// Insert single row
let result = QueryBuilder::new()
    .insert_into("users")
    .values(
        vec!["name", "email", "age"],
        vec!["Bob", "bob@example.com", 30]
    )
    .exec(conn)
    .unwrap();

println!("Inserted with ID: {}", result.last_insert_id);
```

### UPDATE Queries

```jounce
use jounce_db::QueryBuilder;

// Update rows
let result = QueryBuilder::new()
    .update("users")
    .set(vec!["email"], vec!["newemail@example.com"])
    .where_eq("id", 1)
    .exec(conn)
    .unwrap();

println!("Updated {} rows", result.rows_affected);

// Update multiple columns
let result = QueryBuilder::new()
    .update("users")
    .set(
        vec!["name", "email"],
        vec!["Alice Smith", "alice.smith@example.com"]
    )
    .where_eq("id", 2)
    .exec(conn)
    .unwrap();
```

### DELETE Queries

```jounce
use jounce_db::QueryBuilder;

// Delete rows
let result = QueryBuilder::new()
    .delete_from("users")
    .where_eq("active", false)
    .exec(conn)
    .unwrap();

println!("Deleted {} rows", result.rows_affected);

// Delete with complex condition
let result = QueryBuilder::new()
    .delete_from("users")
    .where("created_at < ?", vec!["2024-01-01"])
    .exec(conn)
    .unwrap();
```

### Transactions

```jounce
use jounce_db::{begin_transaction, commit, rollback};

// Begin transaction
begin_transaction(conn).unwrap();

// Execute queries
let result1 = execute(conn, "INSERT INTO users (name) VALUES ('Alice')");
let result2 = execute(conn, "INSERT INTO posts (user_id, title) VALUES (1, 'Hello')");

// Commit if successful
match (result1, result2) {
    (Ok(_), Ok(_)) => {
        commit(conn).unwrap();
        println!("Transaction committed");
    },
    _ => {
        rollback(conn).unwrap();
        println!("Transaction rolled back");
    }
}

// Transaction with query builder
begin_transaction(conn).unwrap();

let user_result = QueryBuilder::new()
    .insert_into("users")
    .values(vec!["name", "email"], vec!["Charlie", "charlie@example.com"])
    .exec(conn);

if user_result.is_ok() {
    commit(conn).unwrap();
} else {
    rollback(conn).unwrap();
}
```

### Working with Results

```jounce
use jounce_db::Row;

// Get values by column name
let row: Row = ...;
let id: int = row.get("id").unwrap();
let name: string = row.get("name").unwrap();
let age: int = row.get("age").unwrap();

// Get values by index
let first_col: int = row.get_at(0).unwrap();
let second_col: string = row.get_at(1).unwrap();

// Handle missing columns
match row.get::<string>("optional_column") {
    Some(value) => println!("Value: {}", value),
    None => println!("Column not found"),
}
```

## Complete Example

```jounce
use jounce_db::{
    DbConfig, create_pool, get_connection, release_connection,
    QueryBuilder, begin_transaction, commit, rollback
};

fn main() {
    // Setup
    let config = DbConfig {
        driver: "postgres",
        host: "localhost",
        port: 5432,
        database: "blog",
        username: "admin",
        password: "secret",
        pool_size: 10,
        timeout: 30,
    };

    let mut pool = create_pool(config);
    let conn = get_connection(pool).unwrap();

    // Create user
    let user_id = create_user(conn, "Alice", "alice@example.com");

    // Create post
    create_post(conn, user_id, "My First Post", "Hello world!");

    // Get user's posts
    let posts = get_user_posts(conn, user_id);
    println!("User has {} posts", posts.len());

    // Cleanup
    release_connection(pool, conn);
}

fn create_user(conn: Connection, name: string, email: string) -> int {
    let result = QueryBuilder::new()
        .insert_into("users")
        .values(vec!["name", "email"], vec![name, email])
        .exec(conn)
        .unwrap();

    return result.last_insert_id;
}

fn create_post(conn: Connection, user_id: int, title: string, content: string) {
    begin_transaction(conn).unwrap();

    let result = QueryBuilder::new()
        .insert_into("posts")
        .values(
            vec!["user_id", "title", "content"],
            vec![user_id, title, content]
        )
        .exec(conn);

    match result {
        Ok(_) => commit(conn).unwrap(),
        Err(_) => rollback(conn).unwrap(),
    }
}

fn get_user_posts(conn: Connection, user_id: int) -> Vec<Row> {
    return QueryBuilder::new()
        .select(vec!["id", "title", "created_at"])
        .from("posts")
        .where_eq("user_id", user_id)
        .order_by("created_at", "DESC")
        .fetch(conn)
        .unwrap();
}
```

## API Reference

### Configuration

- `DbConfig` - Database configuration struct

### Connection Pool

- `create_pool(config: DbConfig) -> ConnectionPool` - Create connection pool
- `get_connection(pool: ConnectionPool) -> Result<Connection, string>` - Get connection
- `release_connection(pool: ConnectionPool, conn: Connection)` - Release connection
- `close_pool(pool: ConnectionPool)` - Close all connections

### Query Execution

- `execute(conn: Connection, sql: string) -> Result<QueryResult, string>` - Execute query
- `execute_prepared(conn: Connection, sql: string, params: Vec<any>) -> Result<QueryResult, string>` - Execute prepared
- `query(conn: Connection, sql: string) -> Result<Vec<Row>, string>` - Query rows
- `query_prepared(conn: Connection, sql: string, params: Vec<any>) -> Result<Vec<Row>, string>` - Query with params
- `query_one(conn: Connection, sql: string) -> Result<Option<Row>, string>` - Query single row

### Transactions

- `begin_transaction(conn: Connection) -> Result<(), string>` - Begin transaction
- `commit(conn: Connection) -> Result<(), string>` - Commit transaction
- `rollback(conn: Connection) -> Result<(), string>` - Rollback transaction

### Query Builder

- `QueryBuilder::new() -> QueryBuilder` - Create builder
- `.select(columns: Vec<string>) -> QueryBuilder` - SELECT columns
- `.select_all() -> QueryBuilder` - SELECT *
- `.from(table: string) -> QueryBuilder` - FROM table
- `.insert_into(table: string) -> QueryBuilder` - INSERT INTO
- `.values(columns: Vec<string>, values: Vec<any>) -> QueryBuilder` - VALUES
- `.update(table: string) -> QueryBuilder` - UPDATE table
- `.set(columns: Vec<string>, values: Vec<any>) -> QueryBuilder` - SET columns
- `.delete_from(table: string) -> QueryBuilder` - DELETE FROM
- `.where(condition: string, params: Vec<any>) -> QueryBuilder` - WHERE clause
- `.where_eq(column: string, value: any) -> QueryBuilder` - WHERE column = value
- `.order_by(column: string, direction: string) -> QueryBuilder` - ORDER BY
- `.limit(n: int) -> QueryBuilder` - LIMIT n
- `.offset(n: int) -> QueryBuilder` - OFFSET n
- `.build() -> (string, Vec<any>)` - Build SQL and params
- `.exec(conn: Connection) -> Result<QueryResult, string>` - Execute
- `.fetch(conn: Connection) -> Result<Vec<Row>, string>` - Fetch rows
- `.fetch_one(conn: Connection) -> Result<Option<Row>, string>` - Fetch one row

### Row Access

- `Row.get<T>(column: string) -> Option<T>` - Get value by column name
- `Row.get_at<T>(index: int) -> Option<T>` - Get value by index

## Best Practices

1. **Use Connection Pooling**: Always use pools for multi-request apps
2. **Prepared Statements**: Use parameters to prevent SQL injection
3. **Transactions**: Wrap related operations in transactions
4. **Resource Cleanup**: Always release connections back to pool
5. **Error Handling**: Handle all Result types properly
6. **Query Builder**: Prefer query builder for complex dynamic queries

## Examples

See `examples/` directory:
- `basic-crud.jnc` - Basic CRUD operations
- `transactions.jnc` - Transaction handling
- `query-builder.jnc` - Advanced query building

## License

MIT

## Version

0.1.0
