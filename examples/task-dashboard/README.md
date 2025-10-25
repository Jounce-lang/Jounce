# Task Management Dashboard

**Multi-Package Integration Example**

This example demonstrates how to build a complete application using multiple Jounce packages working together seamlessly.

## Packages Used

This app integrates **6 Jounce packages**:

1. **jounce-auth** (v0.1.0) - User authentication and session management
2. **jounce-db** (v0.1.0) - Database operations with query builder
3. **jounce-cache** (v0.1.0) - LRU caching for performance
4. **jounce-ui** (v0.1.0) - UI components (Button, Input, Card, Toast)
5. **jounce-logger** (v0.1.0) - Structured logging
6. **jounce-theme** (v0.1.0) - Dark/light mode theming

## Features

### Authentication (jounce-auth)
- JWT token generation
- Session management with TTL
- Secure user login

### Data Persistence (jounce-db)
- SQLite database with connection pooling
- Query builder for type-safe queries
- CRUD operations for tasks

### Performance (jounce-cache)
- LRU cache for frequently accessed tasks
- 5-minute TTL on cached data
- Automatic cache invalidation on updates

### UI Components (jounce-ui)
- Styled buttons with variants (Primary, Success, Ghost)
- Text and password inputs with labels
- Card components for task display
- Toast notifications for user feedback

### Logging (jounce-logger)
- Structured JSON logging
- Log levels (Info, Warn)
- User action tracking

### Theming (jounce-theme)
- Dark/light mode toggle
- System preference detection
- Persistent theme selection

## Application Flow

```
1. App Initialization
   ├─ Logger setup (jounce-logger)
   ├─ Theme detection (jounce-theme)
   └─ Database setup (jounce-db)

2. User Login
   ├─ Credentials input (jounce-ui)
   ├─ JWT creation (jounce-auth)
   ├─ Session creation (jounce-auth)
   └─ Success toast (jounce-ui)

3. Load Tasks
   ├─ Check cache first (jounce-cache)
   ├─ Query database if cache miss (jounce-db)
   ├─ Store in cache (jounce-cache)
   └─ Log action (jounce-logger)

4. Create Task
   ├─ Insert to database (jounce-db)
   ├─ Invalidate cache (jounce-cache)
   └─ Log creation (jounce-logger)

5. Toggle Task
   ├─ Update database (jounce-db)
   ├─ Invalidate cache (jounce-cache)
   └─ Log toggle (jounce-logger)

6. Theme Toggle
   ├─ Switch mode (jounce-theme)
   ├─ Show toast (jounce-ui)
   └─ Log change (jounce-logger)
```

## Running the Example

### Prerequisites

```bash
# Install required packages
jnc pkg add jounce-auth
jnc pkg add jounce-db
jnc pkg add jounce-cache
jnc pkg add jounce-ui
jnc pkg add jounce-logger
jnc pkg add jounce-theme
```

### Compile and Run

```bash
cd examples/task-dashboard
jnc compile main.jnc
node dist/main.js
```

### Expected Output

The application will:
1. Initialize logger, theme, and database
2. Log in a demo user
3. Show a success toast notification
4. Create a sample task
5. Load tasks from database (with caching)
6. Render the application UI as HTML
7. Log all actions in JSON format

## Code Highlights

### Authentication Integration

```jounce
use jounce_auth::{JwtManager, SessionManager};

fn login(username: string, password: string) -> Option<Session> {
    let jwt_manager = JwtManager::new("secret-key");
    let mut session_manager = SessionManager::new();

    let token = jwt_manager.create_token(claims, 3600);
    let session = session_manager.create_session(user_id, {"token": token});

    return Some(session);
}
```

### Database + Cache Integration

```jounce
use jounce_db::{QueryBuilder, ConnectionPool};
use jounce_cache::Cache;

fn load_tasks(user_id: string, pool: ConnectionPool, cache: Cache<Array<Task>>) -> Array<Task> {
    // Check cache first
    let cached = cache.get("tasks_" + user_id);
    if cached.is_some() { return cached.unwrap(); }

    // Cache miss - query database
    let query = QueryBuilder::new()
        .select(["*"])
        .from("tasks")
        .where_clause("user_id", "=", user_id)
        .build();

    let tasks = pool.query(query);

    // Store in cache
    cache.set("tasks_" + user_id, tasks, Some(300));

    return tasks;
}
```

### UI + Theme Integration

```jounce
use jounce_ui::{Button, ButtonVariant, Toast, ToastType};
use jounce_theme::{ThemeManager, ThemeMode};

fn toggle_theme() {
    let theme_manager = ThemeManager::new();
    let new_mode = theme_manager.toggle_mode();

    Toast::new("Theme changed to " + new_mode.to_string())
        .with_type(ToastType::Info)
        .with_duration(2000)
        .show();
}
```

### Structured Logging

```jounce
use jounce_logger::{Logger, LogLevel, OutputFormat};

let logger = Logger::new("TaskDashboard")
    .with_level(LogLevel::Info)
    .with_format(OutputFormat::Json);

logger.info("Task created", {"task_id": task_id, "user": user_id});
```

## Architecture Benefits

### Separation of Concerns
Each package handles a specific domain:
- Auth handles security
- DB handles persistence
- Cache handles performance
- UI handles presentation
- Logger handles observability
- Theme handles appearance

### Reusability
All components are package-based and reusable:
- Use the same auth package across all apps
- Share UI components between projects
- Consistent logging format everywhere

### Maintainability
Clear boundaries make the code easy to maintain:
- Update cache strategy without touching DB code
- Change UI library without affecting business logic
- Swap authentication method independently

### Performance
Smart caching reduces database load:
- 5-minute cache TTL for tasks
- LRU eviction prevents memory bloat
- Cache invalidation on updates ensures consistency

## Extending the Example

### Add More Features

1. **User Registration** (jounce-auth)
   - Create new user accounts
   - Hash passwords
   - Email verification

2. **Task Categories** (jounce-db)
   - Add category field to tasks
   - Filter tasks by category
   - Category management UI

3. **Animations** (jounce-animate)
   - Fade in new tasks
   - Slide out completed tasks
   - Spring animation for theme toggle

4. **API Layer** (jounce-rpc)
   - Expose RPC endpoints
   - Add request/response logging
   - Implement retry logic

5. **Data Utilities** (jounce-utils)
   - Format dates nicely
   - Slugify task titles for URLs
   - Group tasks by date

6. **Documentation** (jounce-docs)
   - Auto-generate API docs
   - Extract code examples
   - Build user guide

## Key Learnings

### Package Integration is Seamless
Jounce packages work together naturally:
- Consistent API design across packages
- Type-safe interfaces
- Clear documentation

### Composition Over Complexity
Build complex apps from simple packages:
- Each package does one thing well
- Combine packages for powerful features
- Easy to understand and debug

### Production-Ready
This example demonstrates patterns for real apps:
- Error handling with Option types
- Logging for observability
- Caching for performance
- Session management for security
- UI feedback with toasts
- Accessibility with proper labels

## Next Steps

1. **Add Tests**
   - Unit tests for business logic
   - Integration tests for package interactions
   - E2E tests for user flows

2. **Deploy**
   - Bundle for production
   - Set up database migrations
   - Configure environment variables

3. **Monitor**
   - Review JSON logs
   - Track cache hit rates
   - Monitor session durations

## License

MIT

## Related Examples

- `counter-app` - Basic reactivity example
- `todo-app-reactive` - Reactive state management
- `form-validation` - Form handling with validation

## Package Versions

- jounce-auth: 0.1.0
- jounce-db: 0.1.0
- jounce-cache: 0.1.0
- jounce-ui: 0.1.0
- jounce-logger: 0.1.0
- jounce-theme: 0.1.0
