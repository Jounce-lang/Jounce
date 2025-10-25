# jounce-auth

Authentication & Authorization package for Jounce applications.

## Features

- ✅ **JWT Tokens** - Create and verify JSON Web Tokens
- ✅ **Session Management** - In-memory sessions with TTL
- ✅ **OAuth 2.0 Helpers** - Authorization flows for OAuth providers
- ✅ **RBAC** - Role-based access control with permissions

## Installation

```bash
jnc pkg add jounce-auth
```

## Usage

### JWT Tokens

```jounce
use jounce_auth::{create_jwt, verify_jwt, JwtClaims};

// Create a token
let claims = JwtClaims {
    sub: "user123",
    exp: unix_timestamp() + 3600, // Expires in 1 hour
    iat: unix_timestamp(),
    iss: "my-app",
    aud: "users",
    roles: vec!["user", "premium"],
};

let token = create_jwt(claims, "my-secret-key");

// Verify a token
match verify_jwt(token, "my-secret-key") {
    Ok(claims) => {
        println!("User ID: {}", claims.sub);
    },
    Err(error) => {
        println!("Invalid token: {}", error);
    }
}
```

### Sessions

```jounce
use jounce_auth::{create_session, get_session, update_session, destroy_session};

// Create a session
let session = create_session("user123", 3600); // 1 hour TTL
println!("Session ID: {}", session.id);

// Update session data
update_session(session.id, "cart_total", "99.99");

// Get session
match get_session(session.id) {
    Some(session) => {
        println!("User: {}", session.user_id);
    },
    None => {
        println!("Session not found or expired");
    }
}

// Destroy session (logout)
destroy_session(session.id);
```

### OAuth 2.0

```jounce
use jounce_auth::{OAuthProvider, oauth_auth_url, oauth_exchange_code};

// Configure OAuth provider (e.g., GitHub)
let github = OAuthProvider {
    name: "GitHub",
    client_id: "your-client-id",
    client_secret: "your-client-secret",
    auth_url: "https://github.com/login/oauth/authorize",
    token_url: "https://github.com/login/oauth/access_token",
    redirect_uri: "https://your-app.com/auth/callback",
};

// Generate authorization URL
let auth_url = oauth_auth_url(
    github,
    "random-state-string",
    vec!["user:email", "repo"]
);

// Redirect user to auth_url...
// After user authorizes, exchange code for token
match oauth_exchange_code(github, code_from_callback) {
    Ok(token) => {
        println!("Access token: {}", token.access_token);
    },
    Err(error) => {
        println!("OAuth error: {}", error);
    }
}
```

### RBAC (Role-Based Access Control)

```jounce
use jounce_auth::{User, has_role, has_permission, authorize};

let user = User {
    id: "user123",
    email: "alice@example.com",
    roles: vec!["user", "editor"],
};

// Check if user has a role
if has_role(user, "editor") {
    println!("User is an editor");
}

// Check if user has permission
if has_permission(user, "posts", "delete") {
    println!("User can delete posts");
}

// Authorize action (returns Result)
match authorize(user, "posts", "delete") {
    Ok(()) => {
        // User authorized, proceed with action
        delete_post(post_id);
    },
    Err(error) => {
        println!("Not authorized: {}", error);
    }
}
```

### Predefined Roles

```jounce
use jounce_auth::{admin_role, user_role, guest_role};

// Admin role - has all permissions (*/*)
let admin = admin_role();

// User role - can read/update own profile
let user = user_role();

// Guest role - read-only access to public resources
let guest = guest_role();
```

## API Reference

### JWT Functions

- `create_jwt(claims: JwtClaims, secret: string) -> string`
- `verify_jwt(token: string, secret: string) -> Result<JwtClaims, string>`
- `is_token_expired(token: string) -> bool`

### Session Functions

- `create_session(user_id: string, ttl_seconds: int) -> Session`
- `get_session(session_id: string) -> Option<Session>`
- `update_session(session_id: string, key: string, value: string) -> bool`
- `destroy_session(session_id: string) -> bool`

### OAuth Functions

- `oauth_auth_url(provider: OAuthProvider, state: string, scopes: Vec<string>) -> string`
- `oauth_exchange_code(provider: OAuthProvider, code: string) -> Result<OAuthToken, string>`
- `oauth_refresh_token(provider: OAuthProvider, refresh_token: string) -> Result<OAuthToken, string>`

### RBAC Functions

- `has_role(user: User, role_name: string) -> bool`
- `has_permission(user: User, resource: string, action: string) -> bool`
- `authorize(user: User, resource: string, action: string) -> Result<(), string>`
- `admin_role() -> Role`
- `user_role() -> Role`
- `guest_role() -> Role`

## Types

```jounce
struct JwtClaims {
    sub: string,      // Subject (user ID)
    exp: int,         // Expiration time
    iat: int,         // Issued at
    iss: string,      // Issuer
    aud: string,      // Audience
    roles: Vec<string>, // User roles
}

struct Session {
    id: string,
    user_id: string,
    data: Map<string, string>,
    created_at: int,
    expires_at: int,
}

struct OAuthProvider {
    name: string,
    client_id: string,
    client_secret: string,
    auth_url: string,
    token_url: string,
    redirect_uri: string,
}

struct User {
    id: string,
    email: string,
    roles: Vec<string>,
}

struct Permission {
    resource: string,
    action: string,
}

struct Role {
    name: string,
    permissions: Vec<Permission>,
}
```

## Security Best Practices

1. **JWT Secrets**: Use strong, random secrets (32+ characters)
2. **Token Expiration**: Keep JWT expiration times short (15-60 minutes)
3. **HTTPS Only**: Always use HTTPS in production
4. **Session Storage**: Replace in-memory sessions with Redis/database for production
5. **OAuth State**: Use random, unguessable state parameters
6. **Permissions**: Follow principle of least privilege

## Examples

See `examples/` directory for complete examples:
- `jwt-auth.jnc` - JWT-based authentication
- `session-auth.jnc` - Session-based authentication
- `oauth-github.jnc` - GitHub OAuth integration
- `rbac-admin-panel.jnc` - Admin panel with RBAC

## License

MIT

## Version

0.1.0
